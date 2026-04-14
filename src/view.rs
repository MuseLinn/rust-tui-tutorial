use crate::app::App;
use crate::components::{hint_popup, navigation};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    // Split area into main content and footer
    let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]);
    let [main_area, footer_area] = layout.areas(area);

    // Render current screen
    match &app.state.screen {
        crate::models::Screen::Welcome => {
            crate::screens::welcome::render(app, frame, main_area);
        }
        crate::models::Screen::LessonMenu => {
            crate::screens::lesson_menu::render(app, frame, main_area);
        }
        crate::models::Screen::Lesson { .. } => {
            crate::screens::lesson::render(app, frame, main_area);
        }
        crate::models::Screen::Exercise { .. } => {
            crate::screens::exercise::render(app, frame, main_area);
        }
        crate::models::Screen::Summary { .. } => {
            crate::screens::summary::render(app, frame, main_area);
        }
    }

    // Footer navigation
    navigation::render(frame, footer_area);

    // Hint popup overlay
    if app.state.popup_visible {
        let hint = match &app.state.screen {
            crate::models::Screen::Exercise { exercise_id, .. } => app
                .manifest
                .phases
                .iter()
                .flat_map(|p| p.lessons.iter())
                .filter_map(|l| l.exercise.as_ref())
                .find(|e| &e.id == exercise_id)
                .and_then(|e| e.hints.first())
                .map(|s| s.as_str())
                .unwrap_or("暂无提示 / No hint available"),
            _ => "按 [?] 关闭提示 / Press [?] to close",
        };
        hint_popup::render(frame, hint);
    }

    // Help overlay
    if app.state.help_visible {
        render_help(frame);
    }

    // Error toast
    if let Some(ref err) = app.state.last_error {
        let error_line = Line::from(err.as_str()).style(Style::default().fg(Color::Red).bold());
        let error_para = Paragraph::new(error_line)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Color::Red),
            )
            .wrap(Wrap { trim: true });
        let error_area = ratatui::layout::Rect {
            x: 0,
            y: 0,
            width: area.width,
            height: 3,
        };
        frame.render_widget(Clear, error_area);
        frame.render_widget(error_para, error_area);
    }
}

fn render_help(frame: &mut Frame) {
    let area = frame.area();
    let help_area = hint_popup::centered_rect(60, 50, area);
    frame.render_widget(Clear, help_area);

    let text = r#"
快捷键 / Shortcuts:
  [q]          退出 / Quit
  [Enter]      继续 / Continue
  [↑] [↓]      滚动 / Scroll
  [?]          提示 / Hint
  [h]          帮助 / Help
  [Ctrl+R]     运行编译 / Run compile
"#;

    let block = Block::default()
        .title(" 帮助 / Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, help_area);
}
