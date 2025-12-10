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

    let system_prompt = "Depois, aplica regras lógicas e padrões simbólicos para detectar contradições, vícios ou obrigações incomuns (parte simbólica)";
    let user_request = "Um exemplo de jurisprudência sobre juros abusivos é o entendimento do Superior Tribunal de Justiça (STJ), segundo o qual a simples estipulação de juros superiores a 12% ao ano não caracteriza, por si só, abusividade, mas a fixação da taxa em patamar acima de uma vez e meia (50%) da média de mercado pode configurar abuso, especialmente em contratos de consumo. Em um caso específico, o STJ determinou que, para reconhecer a abusividade, é necessário analisar as peculiaridades do contrato e o contexto da relação jurídica, não bastando apenas o valor nominal da taxa, mas sim sua proporção em relação à média divulgada pelo Banco Central do Brasil.";

    info!("System Prompt: {}", system_prompt);
    info!("User Request: {}", user_request);

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
        model: "openai/gpt-oss-20b".to_string(), // Ou outro modelo Groq de sua escolha
        messages,
        temperature: 1.0,
        max_completion_tokens: 8192,
        top_p: 1.0,
        stream: false,
        reasoning_effort: "medium".to_string(),
        stop: None,
    };

    info!("Corpo da requisição para a API Groq construído.");

    let client = reqwest::Client::new();
    let response_result = client.post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", groq_api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            error!("Erro ao enviar requisição para a API Groq: {}", e);
            e
        })?;

    let response_text = response_result.text().await.map_err(|e| {
        error!("Erro ao ler o corpo da resposta da API Groq: {}", e);
        e
    })?;

    info!("Corpo da resposta bruta da API Groq: {}", response_text);

    let response: ChatCompletionResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            error!("Erro ao desserializar resposta da API Groq: {}. Resposta bruta: {}", e, response_text);
            e
        })?;

    info!("Resposta da API Groq recebida e desserializada com sucesso.");
    println!("Resposta da LLM do Groq: {}", response.choices[0].message.content);

    Ok(())
}
