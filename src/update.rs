use crate::action::Action;
use crate::app::App;
use crate::models::Screen;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn update(app: &mut App, action: Action) -> Option<Action> {
    match action {
        Action::Quit => return Some(Action::Quit),
        Action::Tick => {}
        Action::Render => {}
        Action::Resize(_, _) => {}
        Action::Key(key) => {
            return handle_key(app, key);
        }
        Action::Mouse(_) => {}
        Action::Navigate(screen) => {
            app.state.screen = screen;
            app.state.scroll_offset = 0;
            app.state.popup_visible = false;
        }
        Action::NextStep => {
            handle_next_step(app);
        }
        Action::PrevStep => {}
        Action::ScrollUp => match app.state.screen {
            Screen::LessonMenu => {
                if app.state.menu_selection > 0 {
                    app.state.menu_selection -= 1;
                }
            }
            _ => {
                if app.state.scroll_offset > 0 {
                    app.state.scroll_offset -= 1;
                }
            }
        },
        Action::ScrollDown => match app.state.screen {
            Screen::LessonMenu => {
                let total = crate::screens::lesson_menu::total_lesson_count(app);
                if app.state.menu_selection + 1 < total {
                    app.state.menu_selection += 1;
                }
            }
            _ => {
                app.state.scroll_offset = app.state.scroll_offset.saturating_add(1);
            }
        },
        Action::ToggleHint => {
            app.state.popup_visible = !app.state.popup_visible;
        }
        Action::ToggleHelp => {
            app.state.help_visible = !app.state.help_visible;
        }
        Action::UpdateCode(code) => {
            if let Screen::Exercise {
                ref mut user_code, ..
            } = app.state.screen
            {
                *user_code = code;
            }
        }
        Action::RunCompile => {
            if let Screen::Exercise { exercise_id, user_code, .. } = app.state.screen.clone()
                && let Some(exercise) = app.manifest.phases.iter()
                    .flat_map(|p| p.lessons.iter())
                    .filter_map(|l| l.exercise.as_ref())
                    .find(|e| e.id == exercise_id)
            {
                let code = user_code.clone();
                let rule = exercise.validation.clone();
                let tx = app.action_tx.clone();
                tokio::spawn(async move {
                    let result = crate::validator::validate_sync(&code, &rule);
                    let _ = tx.send(Action::CompileDone(result));
                });
            }
        }
        Action::CompileDone(result) => {
            if let Screen::Exercise {
                ref mut compile_result,
                ..
            } = app.state.screen
            {
                *compile_result = Some(result);
            }
        }
        Action::SelectLesson(id) => {
            for phase in &app.manifest.phases {
                for lesson in &phase.lessons {
                    if lesson.id == id {
                        app.state.screen = Screen::Lesson {
                            phase_id: phase.id.clone(),
                            lesson_id: id,
                            step_index: 0,
                        };
                        app.state.scroll_offset = 0;
                        return None;
                    }
                }
            }
        }
        Action::SelectExercise(id) => {
            let mut starter = None;
            'outer: for phase in &app.manifest.phases {
                for lesson in &phase.lessons {
                    if let Some(ref ex) = lesson.exercise && ex.id == id {
                        starter = Some(ex.starter_code.clone());
                        app.state.screen = Screen::Exercise {
                            exercise_id: id,
                            user_code: ex.starter_code.clone(),
                            compile_result: None,
                        };
                        app.state.scroll_offset = 0;
                        app.state.popup_visible = false;
                        break 'outer;
                    }
                }
            }
            if let Some(s) = starter {
                init_textarea(app, &s);
            }
        }
        Action::SaveProgress => {}
        Action::LoadProgress => {}
        Action::Error(msg) => {
            app.state.last_error = Some(msg);
        }
    }
    None
}

fn handle_key(app: &mut App, key: crossterm::event::KeyEvent) -> Option<Action> {
    use crossterm::event::KeyCode;
    match &app.state.screen {
        Screen::Welcome => if key.code == KeyCode::Enter {
            app.state.screen = Screen::LessonMenu;
        },
        Screen::LessonMenu => if key.code == KeyCode::Enter {
            let mut flat_index = 0;
            for phase in &app.manifest.phases {
                for lesson in &phase.lessons {
                    if flat_index == app.state.menu_selection {
                        app.state.screen = Screen::Lesson {
                            phase_id: phase.id.clone(),
                            lesson_id: lesson.id.clone(),
                            step_index: 0,
                        };
                        app.state.scroll_offset = 0;
                        return None;
                    }
                    flat_index += 1;
                }
            }
        },
        Screen::Exercise { .. } => {
            if let Some(ref mut textarea) = app.textarea {
                let _ = textarea.input(key);
                let code = textarea.lines().join("\n");
                if let Screen::Exercise { ref mut user_code, .. } = app.state.screen {
                    *user_code = code;
                }
            }
        }
        _ => {}
    }
    None
}

fn handle_next_step(app: &mut App) {
    match &mut app.state.screen {
        Screen::Welcome => {
            app.state.screen = Screen::LessonMenu;
        }
        Screen::Lesson {
            phase_id,
            lesson_id,
            ..
        } => {
            let phase_id = phase_id.clone();
            let lesson_id = lesson_id.clone();
            let starter = app.manifest.phases.iter()
                .find(|p| p.id == phase_id)
                .and_then(|phase| phase.lessons.iter().find(|l| l.id == lesson_id))
                .and_then(|lesson| lesson.exercise.as_ref())
                .map(|ex| ex.starter_code.clone());

            if let Some(starter) = starter {
                app.state.screen = Screen::Exercise {
                    exercise_id: format!("{}_{}", phase_id, lesson_id),
                    user_code: starter.clone(),
                    compile_result: None,
                };
                init_textarea(app, &starter);
                return;
            }
            app.progress.completed_lessons.insert(lesson_id.clone());
            let _ = app.db.mark_lesson_complete(&lesson_id);
            app.state.screen = Screen::LessonMenu;
        }
        Screen::Exercise { exercise_id, .. } => {
            let ex_id = exercise_id.clone();
            app.progress.completed_exercises.insert(ex_id.clone());
            let _ = app.db.mark_exercise_complete(&ex_id, None);
            app.state.screen = Screen::LessonMenu;
            app.textarea = None;
        }
        Screen::Summary { .. } => {
            app.state.screen = Screen::LessonMenu;
        }
        _ => {}
    }
}

fn init_textarea(app: &mut App, starter_code: &str) {
    let lines: Vec<String> = starter_code.lines().map(|s| s.to_string()).collect();
    let mut textarea = tui_textarea::TextArea::new(lines);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Code Editor "),
    );
    textarea.set_style(Style::default().fg(Color::Cyan));
    app.textarea = Some(textarea);
}
