use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_completion_tokens: u32,
    top_p: f32,
    stream: bool,
    reasoning_effort: String,
    stop: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

pub struct GroqClient {
    api_key: String,
    client: reqwest::Client,
}

impl GroqClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn chat_completion(&self, system_prompt: &str, user_request: &str) -> Result<String, Box<dyn std::error::Error>> {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_request.to_string(),
            },
        ];

        let request_body = ChatCompletionRequest {
            model: "openai/gpt-oss-20b".to_string(),
            messages,
            temperature: 1.0,
            max_completion_tokens: 8192,
            top_p: 1.0,
            stream: false,
            reasoning_effort: "medium".to_string(),
            stop: None,
        };

        let response_result = self.client.post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_text = response_result.text().await?;
        let response: ChatCompletionResponse = serde_json::from_str(&response_text)?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No choices in response".into())
        }
    }
}
