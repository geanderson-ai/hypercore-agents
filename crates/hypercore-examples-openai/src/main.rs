use hypercore::Agent;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let policy = hypercore_openai::boxed_openai("DUMMY_KEY");
    let tool = Arc::new(hypercore_tools::HttpTool::new());
    let memory = Arc::new(hypercore_memory_sqlite::SqliteMemory::new("memory.db"));

    let agent = Agent::builder("hello")
        .with_policy(policy)
        .with_tool(tool)
        .with_memory(memory)
        .build()
        .unwrap();

    let out = agent.run("Resuma este contrato em 3 pontos").await.unwrap();
    println!("Hypercore output: {}", out);
}
