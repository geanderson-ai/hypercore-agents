use async_trait::async_trait;
use std::sync::Arc;

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

pub struct Agent {
    name: String,
    policy: Arc<dyn Policy>,
    tools: Vec<Arc<dyn Tool>>,
    memory: Arc<dyn Memory>,
}

impl Agent {
    pub fn builder(name: &str) -> AgentBuilder {
        AgentBuilder::new(name)
    }
}

pub struct AgentBuilder {
    name: String,
    policy: Option<Arc<dyn Policy>>,
    tools: Vec<Arc<dyn Tool>>,
    memory: Option<Arc<dyn Memory>>,
}

impl AgentBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            policy: None,
            tools: Vec::new(),
            memory: None,
        }
    }

    pub fn with_policy(mut self, policy: Arc<dyn Policy>) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn with_tool(mut self, tool: Arc<dyn Tool>) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn with_memory(mut self, memory: Arc<dyn Memory>) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn build(self) -> Result<Agent, &'static str> {
        Ok(Agent {
            name: self.name,
            policy: self.policy.ok_or("policy required")?,
            tools: self.tools,
            memory: self.memory.ok_or("memory required")?,
        })
    }
}

impl Agent {
    pub async fn run(&self, input: &str) -> HyperResult<String> {
        let _ctx = self.memory.retrieve(input, 5).await?;
        let decision = self.policy.decide(input, self.memory.as_ref()).await?;

        if decision.starts_with("TOOL:") {
            let arg = decision.trim_start_matches("TOOL:").trim();
            if let Some(tool) = self.tools.get(0) {
                return tool.call(arg).await;
            }
            return Err(HyperError::Tool("no tool available".into()));
        }

        Ok(decision)
    }
}
