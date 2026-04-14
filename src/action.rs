use crate::models::{CompileResult, Screen};
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Action {
    Tick,
    Render,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Quit,
    Navigate(Screen),
    NextStep,
    PrevStep,
    ScrollUp,
    ScrollDown,
    ToggleHint,
    ToggleHelp,
    UpdateCode(String),
    RunCompile,
    CompileDone(CompileResult),
    SelectLesson(String),
    SelectExercise(String),
    SaveProgress,
    LoadProgress,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn test_action_enum() {
        let _actions = vec![
            Action::Tick,
            Action::Render,
            Action::Key(KeyEvent::from(KeyCode::Enter)),
            Action::Quit,
            Action::Navigate(Screen::Welcome),
            Action::CompileDone(CompileResult::Success),
        ];
    }
}
