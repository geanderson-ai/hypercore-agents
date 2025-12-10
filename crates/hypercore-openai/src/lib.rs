use hypercore::{HyperError, HyperResult, Memory, Policy};
use async_trait::async_trait;
use std::sync::Arc;
use serde_json::json;

pub struct OpenAIPolicy {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIPolicy {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { 
            api_key: api_key.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Policy for OpenAIPolicy {
    async fn decide(&self, input: &str, _memory: &dyn Memory) -> HyperResult<String> {
        // In a real implementation, this would call the OpenAI API.
        // For now, we simulate a response that might contain facts.
        
        let response_text = if input.contains("Validar contrato") {
            json!([
                { "entity": "contract_123", "attribute": "status", "value": "active", "confidence": 99 },
                { "entity": "contract_123", "attribute": "amount", "value": "50000", "confidence": 95 }
            ]).to_string()
        } else {
             format!("(Simulated OpenAI Response for: {})", input)
        };

        Ok(response_text)
    }

}

impl OpenAIPolicy {
    pub fn interpret_as_facts(&self, text: &str) -> Vec<hypercore_symbolic::Fact> {
        let parsed: Result<Vec<serde_json::Value>, _> = serde_json::from_str(text);
        
        if let Ok(json_facts) = parsed {
            json_facts.into_iter().filter_map(|j| {
                Some(hypercore_symbolic::Fact {
                    entity: j["entity"].as_str()?.to_string(),
                    attribute: j["attribute"].as_str()?.to_string(),
                    value: j["value"].as_str()?.to_string(),
                    confidence: j["confidence"].as_u64()? as u8,
                })
            }).collect()
        } else {
            vec![]
        }
    }
}

pub fn boxed_openai(api_key: impl Into<String>) -> Arc<dyn Policy> {
    Arc::new(OpenAIPolicy::new(api_key))
}