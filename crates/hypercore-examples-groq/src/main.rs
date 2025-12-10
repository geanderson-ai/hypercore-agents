use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use log::{info, error};
use async_trait::async_trait;
use hypercore::{Agent, Tool, Memory, core::{Hypercore, HypercoreBuilder, Storage, HypercoreError}, Message as HypercoreMessage};
use hypercore_memory_sqlite::storage::SqliteStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

struct GroqAgent {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

#[async_trait]
impl Agent for GroqAgent {
    fn name(&self) -> String {
        "GroqAgent".to_string()
    }

    async fn chat(&self, messages: Vec<HypercoreMessage>, _tools: &[Box<dyn Tool>], _memory: &mut dyn Memory) -> Result<HypercoreMessage, HypercoreError> {
        let groq_messages: Vec<Message> = messages.into_iter().map(|msg| Message {
            role: msg.role,
            content: msg.content,
        }).collect();

        let request_body = ChatCompletionRequest {
            model: self.model.clone(),
            messages: groq_messages,
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
            .await
            .map_err(|e| HypercoreError::AgentError(format!("Erro ao enviar requisição para a API Groq: {}", e)))?;

        let response_text = response_result.text().await.map_err(|e| HypercoreError::AgentError(format!("Erro ao ler o corpo da resposta da API Groq: {}", e)))?;

        let response: ChatCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| HypercoreError::AgentError(format!("Erro ao desserializar resposta da API Groq: {}. Resposta bruta: {}", e, response_text)))?;

        let agent_message = response.choices[0].message.clone();

        Ok(HypercoreMessage {
            role: agent_message.role,
            content: agent_message.content,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Aplicação iniciada.");
    dotenv().ok();

    let groq_api_key = match env::var("GROQ_API_KEY") {
        Ok(key) => {
            info!("GROQ_API_KEY carregada com sucesso.");
            key
        },
        Err(e) => {
            error!("Erro ao carregar GROQ_API_KEY: {}. Certifique-se de que está definida no arquivo .env", e);
            return Err(e.into());
        },
    };

    let client = reqwest::Client::new();
    let groq_agent = GroqAgent {
        client,
        api_key: groq_api_key,
        model: "llama3-8b-8192".to_string(), // Ou outro modelo Groq de sua escolha
    };

    let storage = SqliteStorage::new("hypercore_groq.db").await?;
    let mut hypercore = HypercoreBuilder::new()
        .with_agent(Box::new(groq_agent))
        .with_storage(Box::new(storage))
        .build().await?;

    let system_prompt = "Depois, aplica regras lógicas e padrões simbólicos para detectar contradições, vícios ou obrigações incomuns (parte simbólica)";
    let user_request = "Um exemplo de jurisprudência sobre juros abusivos é o entendimento do Superior Tribunal de Justiça (STJ), segundo o qual a simples estipulação de juros superiores a 12% ao ano não caracteriza, por si só, abusividade, mas a fixação da taxa em patamar acima de uma vez e meia (50%) da média de mercado pode configurar abuso, especialmente em contratos de consumo. Em um caso específico, o STJ determinou que, para reconhecer a abusividade, é necessário analisar as peculiaridades do contrato e o contexto da relação jurídica, não bastando apenas o valor nominal da taxa, mas sim sua proporção em relação à média divulgada pelo Banco Central do Brasil.";

    info!("System Prompt: {}", system_prompt);
    info!("User Request: {}", user_request);

    let messages = vec![
        HypercoreMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        HypercoreMessage {
            role: "user".to_string(),
            content: user_request.to_string(),
        },
    ];

    let response = hypercore.chat(messages, &[]).await?;

    info!("Resposta da API Groq recebida e desserializada com sucesso.");
    println!("Resposta da LLM do Groq: {}", response.content);

    Ok(())
}
