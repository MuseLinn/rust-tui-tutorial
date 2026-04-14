use rusqlite::{params, Connection, OptionalExtension, Result};
use std::collections::HashSet;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

#[allow(dead_code)]
impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;",
        )?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "            CREATE TABLE IF NOT EXISTS lessons (
                slug TEXT PRIMARY KEY,
                title TEXT NOT NULL DEFAULT '',
                completed_at TEXT
            );

            CREATE TABLE IF NOT EXISTS exercises (
                slug TEXT PRIMARY KEY,
                lesson_slug TEXT NOT NULL DEFAULT '',
                title TEXT NOT NULL DEFAULT '',
                completed_at TEXT,
                attempts INTEGER NOT NULL DEFAULT 0,
                last_code TEXT
            );

            CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_exercises_lesson
                ON exercises(lesson_slug);",
        )?;
        Ok(())
    }

    pub fn mark_lesson_complete(&self, slug: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO lessons (slug, completed_at)
             VALUES (?1, datetime('now'))
             ON CONFLICT(slug) DO UPDATE SET completed_at = excluded.completed_at",
            params![slug],
        )?;
        Ok(())
    }

    pub fn mark_exercise_complete(&self, slug: &str, code: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO exercises (slug, lesson_slug, title, completed_at, attempts, last_code)
             VALUES (?1, '', '', datetime('now'), 1, ?2)
             ON CONFLICT(slug) DO UPDATE SET
                 completed_at = excluded.completed_at,
                 attempts = attempts + 1,
                 last_code = COALESCE(excluded.last_code, last_code)",
            params![slug, code],
        )?;
        Ok(())
    }

    pub fn get_completed_lessons(&self) -> Result<HashSet<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT slug FROM lessons WHERE completed_at IS NOT NULL")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<HashSet<String>>>()
    }

    pub fn get_completed_exercises(&self) -> Result<HashSet<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT slug FROM exercises WHERE completed_at IS NOT NULL")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<HashSet<String>>>()
    }

    pub fn save_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO meta (key, value)
             VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        self.conn
            .query_row(
                "SELECT value FROM meta WHERE key = ?1",
                params![key],
                |row| row.get::<_, String>(0),
            )
            .optional()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_crud() {
        let tmp = tempfile::tempdir().unwrap();
        let db = Database::open(tmp.path().join("test.db")).unwrap();

        db.mark_lesson_complete("l1").unwrap();
        let lessons = db.get_completed_lessons().unwrap();
        assert!(lessons.contains("l1"));

        db.mark_exercise_complete("e1", Some("fn main() {}"))
            .unwrap();
        let exercises = db.get_completed_exercises().unwrap();
        assert!(exercises.contains("e1"));

        db.save_meta("current_phase", "p1").unwrap();
        assert_eq!(
            db.get_meta("current_phase").unwrap(),
            Some("p1".to_string())
        );
    }
}
