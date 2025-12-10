use async_trait::async_trait;


pub type HyperResult<T> = Result<T, HyperError>;

#[derive(thiserror::Error, Debug)]
pub enum HyperError {
    #[error("llm error: {0}")]
    Llm(String),
    #[error("tool error: {0}")]
    Tool(String),
    #[error("other: {0}")]
    Other(String),
}

#[async_trait]
pub trait Policy: Send + Sync {
    async fn decide(&self, input: &str, memory: &dyn Memory) -> HyperResult<String>;
}

#[async_trait]
pub trait Tool: Send + Sync {
    async fn call(&self, input: &str) -> HyperResult<String>;
}

#[async_trait]
pub trait Memory: Send + Sync {
    async fn retrieve(&self, query: &str, k: usize) -> HyperResult<Vec<String>>;
    async fn store(&self, key: &str, value: &str) -> HyperResult<()>;
}

// Orchestrator skeleton for future use
pub struct Orchestrator {
    // placeholders
}
