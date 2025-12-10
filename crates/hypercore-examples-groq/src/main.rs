use std::env;
use dotenv::dotenv;
use log::{info, error};
use aigroq::GroqClient;

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

    let client = GroqClient::new(groq_api_key);

    let system_prompt = "Depois, aplica regras lógicas e padrões simbólicos para detectar contradições, vícios ou obrigações incomuns (parte simbólica)";
    let user_request = "Um exemplo de jurisprudência sobre juros abusivos é o entendimento do Superior Tribunal de Justiça (STJ), segundo o qual a simples estipulação de juros superiores a 12% ao ano não caracteriza, por si só, abusividade, mas a fixação da taxa em patamar acima de uma vez e meia (50%) da média de mercado pode configurar abuso, especialmente em contratos de consumo. Em um caso específico, o STJ determinou que, para reconhecer a abusividade, é necessário analisar as peculiaridades do contrato e o contexto da relação jurídica, não bastando apenas o valor nominal da taxa, mas sim sua proporção em relação à média divulgada pelo Banco Central do Brasil.";

    info!("System Prompt: {}", system_prompt);
    info!("User Request: {}", user_request);

    match client.chat_completion(system_prompt, user_request).await {
        Ok(response) => {
            info!("Resposta da API Groq recebida e desserializada com sucesso.");
            println!("Resposta da LLM do Groq: {}", response);
        },
        Err(e) => {
            error!("Erro na chamada Groq: {}", e);
        }
    }

    Ok(())
}
