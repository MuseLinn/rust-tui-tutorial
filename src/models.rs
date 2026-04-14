use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LessonManifest {
    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Phase {
    pub id: String,
    pub title: String,
    pub description: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub content_md: String,
    pub exercise: Option<Exercise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub instructions: String,
    pub starter_code: String,
    pub hints: Vec<String>,
    pub validation: ValidationRule,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "param", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum ValidationRule {
    MustCompile,
    MustCompileWithOutput(String),
    MustContainPattern(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileResult {
    Success,
    CompileError { stderr: String },
    RuntimeMismatch { expected: String, got: String },
    PatternMismatch { expected: String },
    Timeout,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct UserProgress {
    pub completed_lessons: HashSet<String>,
    pub completed_exercises: HashSet<String>,
    pub current_phase: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Screen {
    Welcome,
    LessonMenu,
    Lesson {
        phase_id: String,
        lesson_id: String,
        step_index: usize,
    },
    Exercise {
        exercise_id: String,
        user_code: String,
        compile_result: Option<CompileResult>,
    },
    Summary {
        phase_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub screen: Screen,
    pub scroll_offset: u16,
    pub popup_visible: bool,
    pub help_visible: bool,
    pub last_error: Option<String>,
    pub menu_selection: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Welcome,
            scroll_offset: 0,
            popup_visible: false,
            help_visible: false,
            last_error: None,
            menu_selection: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_construct() {
        let _manifest = LessonManifest { phases: vec![] };
        let _phase = Phase {
            id: "p1".into(),
            title: "Test".into(),
            description: "Desc".into(),
            lessons: vec![],
        };
        let _lesson = Lesson {
            id: "l1".into(),
            title: "Lesson".into(),
            content_md: "# Hello".into(),
            exercise: None,
        };
        let _exercise = Exercise {
            id: "e1".into(),
            instructions: "Do this".into(),
            starter_code: "fn main() {}".into(),
            hints: vec![],
            validation: ValidationRule::MustCompile,
        };
        let _progress = UserProgress::default();
        let _screen = Screen::Welcome;
    }
}
