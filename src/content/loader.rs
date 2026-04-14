use crate::models::{LessonManifest, Phase};

macro_rules! load_phase {
    ($file:expr) => {
        toml::from_str(include_str!($file)).expect(concat!("failed to parse ", $file))
    };
}

pub fn load_embedded_manifest() -> LessonManifest {
    let phases: Vec<Phase> = vec![
        load_phase!("phase01_basics.toml"),
        load_phase!("phase02_ownership.toml"),
        load_phase!("phase03_types.toml"),
        load_phase!("phase04_collections.toml"),
        load_phase!("phase05_abstractions.toml"),
        load_phase!("phase06_advanced.toml"),
    ];
    LessonManifest { phases }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_manifest() {
        let manifest = load_embedded_manifest();
        assert_eq!(manifest.phases.len(), 6);
    }
}
