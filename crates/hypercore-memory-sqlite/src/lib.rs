use hypercore::{HyperError, HyperResult, Memory};
use hypercore_symbolic::Fact;
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

        // New table for structured facts
        conn.execute(
            "CREATE TABLE IF NOT EXISTS facts (
                entity TEXT, 
                attribute TEXT, 
                value TEXT, 
                confidence INTEGER,
                UNIQUE(entity, attribute, value)
            )",
            [],
        ).unwrap();

        Self { db_path: p }
    }

    // Symbolic extensions
    pub async fn add_fact(&self, fact: &Fact) -> HyperResult<()> {
        let conn = Connection::open(&self.db_path).map_err(to_err)?;
        conn.execute(
            "INSERT OR IGNORE INTO facts (entity, attribute, value, confidence) VALUES (?1, ?2, ?3, ?4)",
            params![fact.entity, fact.attribute, fact.value, fact.confidence],
        ).map_err(to_err)?;
        Ok(())
    }

    pub async fn get_facts_by_entity(&self, entity: &str) -> HyperResult<Vec<Fact>> {
         let conn = Connection::open(&self.db_path).map_err(to_err)?;
         let mut stmt = conn.prepare("SELECT entity, attribute, value, confidence FROM facts WHERE entity = ?1").map_err(to_err)?;
         
         let fact_iter = stmt.query_map(params![entity], |row| {
             Ok(Fact {
                 entity: row.get(0)?,
                 attribute: row.get(1)?,
                 value: row.get(2)?,
                 confidence: row.get(3)?,
             })
         }).map_err(to_err)?;
         
         let mut facts = Vec::new();
         for fact in fact_iter {
             facts.push(fact.map_err(to_err)?);
         }
         Ok(facts)
    }
}

#[async_trait]
impl Memory for SqliteMemory {
    async fn retrieve(&self, _query: &str, _k: usize) -> HyperResult<Vec<String>> {
        // Basic implementation for now, maybe fuzzy search later
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
