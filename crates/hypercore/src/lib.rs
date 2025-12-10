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
    // In the future this will hold connections to all components.
    // For now, it's a lightweight wrapper around the example flow.
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {}
    }
    
    // Simple placeholder for the high-level reasoning method
    pub async fn reason(&self, goal: &str) -> HyperResult<ReasoningResult> {
       // This would be where `hypercore-hybrid` is invoked.
       // Since crates can't easily be circular, we might not link it directly here yet 
       // without feature flags or dependency reorganization (as hypercore is the core crate).
       // For this MVP step, we will return a mock result to satify the interface contract.
       
       Ok(ReasoningResult {
           conclusion: format!("Processed goal: {}", goal),
           trace: vec!["Initialized".to_string(), "Analyzed".to_string(), "Concluded".to_string()]
       })
    }
}

pub struct ReasoningResult {
    pub conclusion: String,
    pub trace: Vec<String>,
}
