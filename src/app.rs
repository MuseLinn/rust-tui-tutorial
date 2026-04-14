use crate::action::Action;
use crate::content;
use crate::db::Database;
use crate::event::{Event, EventHandler};
use crate::models::{AppState, LessonManifest, UserProgress};
use crate::tui;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use std::path::PathBuf;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct App {
    pub state: AppState,
    pub manifest: LessonManifest,
    pub progress: UserProgress,
    pub db: Database,
    pub action_tx: UnboundedSender<Action>,
    pub action_rx: UnboundedReceiver<Action>,
    pub textarea: Option<tui_textarea::TextArea<'static>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let manifest = content::load_manifest();
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rust-tui-tutorial");
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("progress.db");
        let db = Database::open(&db_path)?;

        let completed_lessons = db.get_completed_lessons().unwrap_or_default();
        let completed_exercises = db.get_completed_exercises().unwrap_or_default();
        let current_phase = db.get_meta("current_phase")?.unwrap_or_default();

        let progress = UserProgress {
            completed_lessons,
            completed_exercises,
            current_phase,
        };

        let (action_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();

        Ok(Self {
            state: AppState::default(),
            manifest,
            progress,
            db,
            action_tx,
            action_rx,
            textarea: None,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = tui::init()?;
        let mut event_handler = EventHandler::new(4.0, 30.0);

        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = tui::restore();
            default_panic(info);
        }));

        let result = self.run_loop(&mut terminal, &mut event_handler).await;

        event_handler.stop();
        tui::restore()?;
        result
    }

    async fn run_loop(
        &mut self,
        terminal: &mut ratatui::DefaultTerminal,
        event_handler: &mut EventHandler,
    ) -> Result<()> {
        loop {
            terminal.draw(|frame| {
                crate::view::render(self, frame);
            })?;

            if let Some(event) = event_handler.next().await {
                let action = map_event_to_action(event, self);
                if let Some(action) = crate::update::update(self, action)
                    && action == Action::Quit
                {
                    break;
                }
            }

            // Drain async actions (e.g., CompileDone)
            while let Ok(action) = self.action_rx.try_recv() {
                if let Some(action) = crate::update::update(self, action)
                    && action == Action::Quit
                {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn map_event_to_action(event: Event, app: &App) -> Action {
    use crate::models::Screen;
    match event {
        Event::Tick => Action::Tick,
        Event::Render => Action::Render,
        Event::Key(key) => {
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                return Action::Quit;
            }
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r') {
                return Action::RunCompile;
            }
            // In Exercise screen, most keys go to the code editor
            if matches!(app.state.screen, Screen::Exercise { .. }) {
                match key.code {
                    KeyCode::Char('?') => return Action::ToggleHint,
                    KeyCode::Esc => return Action::Navigate(Screen::LessonMenu),
                    _ => return Action::Key(key),
                }
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
                KeyCode::Char('?') => Action::ToggleHint,
                KeyCode::Char('h') | KeyCode::Char('H') => Action::ToggleHelp,
                KeyCode::Enter => Action::NextStep,
                KeyCode::Up => Action::ScrollUp,
                KeyCode::Down => Action::ScrollDown,
                _ => Action::Key(key),
            }
        }
        Event::Mouse(mouse) => Action::Mouse(mouse),
        Event::Resize(w, h) => Action::Resize(w, h),
    }
}
