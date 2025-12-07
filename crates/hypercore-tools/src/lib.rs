use hypercore::{HyperResult, Tool};
use async_trait::async_trait;

pub struct HttpTool {}

impl HttpTool {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl Tool for HttpTool {
    async fn call(&self, input: &str) -> HyperResult<String> {
        Ok(format!("HTTP_TOOL_CALLED with: {}", input))
    }
}
