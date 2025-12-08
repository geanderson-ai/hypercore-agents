use pyo3::prelude::*;
use pyo3::types::{PyDict};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use serde_json::Value;
use tokio::runtime::Runtime;
use anyhow::Result;

// Runtime global para executar futures
static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create tokio runtime")
});

static TOOLS: Lazy<Mutex<HashMap<String, PyObject>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[pyfunction]
fn register_tool(py: Python, name: String, func: PyObject) -> PyResult<()> {
    TOOLS.lock().unwrap().insert(name, func);
    Ok(())
}

#[pyfunction]
fn call_tool(py: Python, name: String, args: &PyAny) -> PyResult<PyObject> {
    let tool = {
        let map = TOOLS.lock().unwrap();
        map.get(&name).cloned()
    }.ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("tool not found"))?;

    // Call Python function with args (args expected to be a dict)
    let res = tool.call1(py, (args,))?;
    Ok(res)
}

#[pyfunction]
fn llm_chat(py: Python, prompt: String) -> PyResult<String> {
    // POC: Bloqueia a thread para simplificar. Prod: use pyo3-asyncio.
    let prompt_clone = prompt.clone();
    let fut = async move {
        // Mock de chamada LLM ou substitua por cliente real
        // Para o POC, apenas simulamos um echo "inteligente"
        // Em prod: reqwest call to GROQ/OpenAI
        
        let client = reqwest::Client::new();
        // Exemplo real comentado para evitar falhas sem API KEY.
        /*
        let payload = serde_json::json!({
            "model": "mixtral-8x7b-32768",
            "messages": [{"role":"user", "content": prompt_clone}]
        });
        let resp = client.post("https://api.groq.com/openai/v1/chat/completions")
            .bearer_auth("GROQ_KEY_AQUI")
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;
        let text = resp["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok::<String, anyhow::Error>(text)
        */
        
        // Simulação pro POC:
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok::<String, anyhow::Error>(format!("LLM (mock) response to: '{}'", prompt_clone))
    };
    
    let result = TOKIO_RT.block_on(fut).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))?;
    Ok(result)
}

#[pyfunction]
fn web_search(py: Python, query: String) -> PyResult<String> {
    let q = query.clone();
    let fut = async move {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.duckduckgo.com/")
            .query(&[("q", q), ("format", "json")])
            .send()
            .await?
            .json::<Value>()
            .await?;
        let result = resp["AbstractText"].as_str().unwrap_or("");
        if result.is_empty() {
             Ok::<String, anyhow::Error>("No abstract found (mock search)".to_string())
        } else {
             Ok::<String, anyhow::Error>(result.to_string())
        }
    };
    let result = TOKIO_RT.block_on(fut).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))?;
    Ok(result)
}

#[pymodule]
fn hypercore_native(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(register_tool, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(call_tool, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(llm_chat, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(web_search, m)?)?;
    Ok(())
}
