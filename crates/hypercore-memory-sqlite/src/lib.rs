use hypercore::{HyperError, HyperResult, Memory};
use async_trait::async_trait;
use rusqlite::{params, Connection};

pub struct SqliteMemory {
    db_path: String,
}

impl SqliteMemory {
    pub fn new(path: impl Into<String>) -> Self {
        let p = path.into();
        let conn = Connection::open(&p).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv (k TEXT PRIMARY KEY, v TEXT NOT NULL)",
            [],
        ).unwrap();
        Self { db_path: p }
    }
}

#[async_trait]
impl Memory for SqliteMemory {
    async fn retrieve(&self, _query: &str, _k: usize) -> HyperResult<Vec<String>> {
        Ok(vec![])
    }

    async fn store(&self, key: &str, value: &str) -> HyperResult<()> {
        let conn = Connection::open(&self.db_path).map_err(to_err)?;
        conn.execute("INSERT OR REPLACE INTO kv (k, v) VALUES (?1, ?2)", params![key, value])
            .map_err(to_err)?;
        Ok(())
    }
}

fn to_err(e: rusqlite::Error) -> HyperError {
    HyperError::Other(format!("{e}"))
}
