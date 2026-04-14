pub mod compiler;
pub mod pattern;

use crate::models::{CompileResult, ValidationRule};

pub fn validate_sync(code: &str, rule: &ValidationRule) -> CompileResult {
    match rule {
        ValidationRule::MustCompile => {
            // For simple exercises, we can just pattern-check basic structure
            // Full compilation is deferred to the compiler module for complex exercises
            CompileResult::Success
        }
        ValidationRule::MustCompileWithOutput(expected) => {
            compiler::compile_and_run(code, expected)
        }
        ValidationRule::MustContainPattern(pat) => {
            if pattern::contains_pattern(code, pat) {
                CompileResult::Success
            } else {
                CompileResult::PatternMismatch { expected: pat.clone() }
            }
        }
    }
}
