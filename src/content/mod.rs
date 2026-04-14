pub mod loader;

use crate::models::LessonManifest;

pub fn load_manifest() -> LessonManifest {
    loader::load_embedded_manifest()
}
