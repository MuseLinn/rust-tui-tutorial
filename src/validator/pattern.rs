use regex::Regex;

pub fn contains_pattern(code: &str, pattern: &str) -> bool {
    // Try regex first; if invalid regex, fall back to simple substring
    if let Ok(re) = Regex::new(pattern) {
        re.is_match(code)
    } else {
        code.contains(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_mut() {
        assert!(contains_pattern("let mut x = 5;", r"let\s+mut\s+"));
    }

    #[test]
    fn test_missing_pattern() {
        assert!(!contains_pattern("let x = 5;", r"let\s+mut\s+"));
    }

    #[test]
    fn test_fallback_substring() {
        assert!(contains_pattern("fn main() {}", "fn main"));
    }
}
