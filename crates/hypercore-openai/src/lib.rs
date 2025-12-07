use hypercore::{HyperResult, Memory, Policy};
use async_trait::async_trait;
use std::sync::Arc;

pub struct OpenAIPolicy {
    api_key: String,
}

impl OpenAIPolicy {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }
}

#[async_trait]
impl Policy for OpenAIPolicy {
    async fn decide(&self, input: &str, _memory: &dyn Memory) -> HyperResult<String> {
        Ok(format!("ECHO: {}", input))
    }
}

pub fn boxed_openai(api_key: impl Into<String>) -> Arc<dyn Policy> {
    Arc::new(OpenAIPolicy::new(api_key))
}