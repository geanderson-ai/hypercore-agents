# HYPERCORE  
### Runtime de Agentes de IA em Rust – Seguro, Rápido e Pronto para Produção

```
 _   _                       _____                
| | | |                     /  __ \               
| |_| |_   _ _ __   ___ _ __| /  \/ ___  _ __ ___ 
|  _  | | | | '_ \ / _ \ '__| |    / _ \| '__/ _ \
| | | | |_| | |_) |  __/ |  | \__/\ (_) | | |  __/
\_| |_/\__, | .__/ \___|_|   \____/\___/|_|  \___|
        __/ | |                                   
       |___/|_|                                   
```

---

## **O que é o Hypercore?**

O **Hypercore** é um *runtime de agentes de IA em Rust*, projetado para desenvolvedores que precisam de:

- Baixa latência real  
- Concorrência massiva  
- Segurança de memória  
- Confiabilidade em produção  
- Integração com LLMs modernas  
- Estrutura modular e extensível  

Ele oferece uma base sólida para construir agentes autônomos, copilotos, pipelines inteligentes e automações robustas — tudo com a previsibilidade e performance do Rust.

**Você escreve o agente.  
O Hypercore garante que ele rode rápido, seguro e sem surpresas.**

---

## **Por que o Hypercore existe?**

Porque frameworks de agentes em Python são ótimos para prototipar, mas péssimos para:

- Escalar  
- Rodar concorrência real  
- Garantir segurança  
- Operar em edge  
- Rodar milhares de agentes simultaneamente  

Rust resolve essas dores, mas faltava um runtime idiomático, conciso, pragmático e modular.  
**O Hypercore nasce para preencher essa lacuna.**

---

## **Filosofia do Projeto**

- **Minimalismo funcional**: apenas o essencial para construir agentes úteis.  
- **Modularidade extrema**: cada parte é um crate separado.  
- **Zero mágica**: sem comportamento implícito. Tudo é explícito.  
- **DX primeiro**: experiência do desenvolvedor importa — exemplos, ergonomia e clareza.  
- **Prod-ready desde o início**: testes, tipagem forte, concorrência segura.  
- **Comunidade acima de tudo**: documentado, simples, extensível.  

---

# **Instalação**

### Via Cargo (em breve no crates.io):

```toml
[dependencies]
hypercore = "0.1"
hypercore-openai = "0.1"
hypercore-tools = "0.1"
hypercore-memory-sqlite = "0.1"
```

---

# **Exemplo mínimo**

```rust
use hypercore::Agent;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let policy = hypercore_openai::boxed_openai("API_KEY");
    let tool = Arc::new(hypercore_tools::HttpTool::new());
    let memory = Arc::new(hypercore_memory_sqlite::SqliteMemory::new("memory.db"));

    let agent = Agent::builder("demo")
        .with_policy(policy)
        .with_tool(tool)
        .with_memory(memory)
        .build()
        .unwrap();

    let out = agent.run("Busque informações sobre IA").await.unwrap();
    println!("{}", out);
}
```

---

# **Arquitetura (MVP)**

### 1. **hypercore**  
Runtime, traits, executor, ciclo do agente.

### 2. **hypercore-openai**  
Adapter simples para políticas baseadas em OpenAI.

### 3. **hypercore-tools**  
Ferramentas internas — HTTP, shell, file operations.

### 4. **hypercore-memory-sqlite**  
Memória básica persistente.

### 5. **hypercore-examples**  
Exemplos end-to-end.

---

# **Manifesto Oficial do Hypercore**

> Construímos agentes de IA que não travam, não vazam memória e não colapsam sob carga.  
>  
> Construímos para quem precisa operar no mundo real — onde latência importa, dados importam e segurança importa.  
>  
> Não queremos ser “mais um framework de agentes”.  
> Queremos ser a camada fundamental onde agentes confiáveis existem.  
>  
> Somos minimalistas, pragmáticos e diretos.  
> Sem mágica. Sem açúcar demais.  
> Apenas um runtime sólido, rápido e previsível.  
>  
> O Hypercore existe para permitir que você construa sistemas inteligentes que **duram**,  
> **escala**,  
> e **funcionam** sob qualquer carga.  
>  
> Rust oferece as fundações.  
> O Hypercore transforma isso em um ecossistema produtivo.  
>  
> Nosso compromisso:  
> **clareza, simplicidade, performance e comunidade.**

---

# **Pitch para o crates.io**

> **Hypercore** — Runtime de agentes de IA em Rust.  
>  
> Seguro, rápido, modular.  
>  
> Construa agentes que realmente executam tarefas, com:  
> - Concorrência massiva  
> - Baixa latência  
> - Políticas baseadas em LLMs  
> - Tool calling  
> - Memória persistente  
>  
> O Hypercore é minimalista, direto e feito para produção.  
> Instale, implemente um `Agent`, adicione uma `Policy`, `Tools` e `Memory`.  
> Pronto: você tem um agente autônomo confiável.

---

# **Roadmap 0.1 → 1.0**

### **v0.1 — MVP Funcional (agora)**
- Runtime mínimo  
- Traits: Agent, Policy, Tool, Memory  
- Executor básico  
- Adapter OpenAI (mock)  
- Tool HTTP simples  
- SQLite memory store  
- Exemplo end-to-end  
- mdBook + CI  

### **v0.2 — Primeiro Release Público**
- Policy OpenAI real  
- Tool HTTP completa  
- Melhorias no ciclo de decisão  
- Logging estruturado  
- Benchmarks oficiais  
- Documentação de API  

### **v0.3 — Ergonomia e Extensibilidade**
- Macro helpers para criar tools  
- Config system  
- Política REACT minimal  
- Suporte a ferramentas encadeadas  
- Melhor tratamento de erros  

### **v0.4 — Performance & Observabilidade**
- Tracing nativo (OpenTelemetry)  
- Mais benchmarks  
- Perfil de memória  
- Loop otimizado  

### **v0.5 — Ecossistema Expande**
- Adapter Llama.cpp / Candle  
- Memory vector store (ANN local)  
- Tool: browser / scraper  
- Tool: file I/O avançado  
- CLI do Hypercore (hyperctl) preview  

### **v0.6 — Segurança e Estabilidade**
- Isolamento de tools  
- Sandboxing leve  
- Configuração de limites (timeouts, quotas)  

### **v0.7 — Multimodal**
- Suporte imagens  
- Suporte áudio  
- Tools multimodais  

### **v0.8 — Workflows**
- Mini orquestrador interno  
- Steps declarativos  
- Retentativas e compensações  

### **v0.9 — Prod Ready**
- Test suite completo  
- Documentação corporativa  
- Integração com containers leves  

### **v1.0 — Lançamento Oficial**
- API estável  
- Conjunto robusto de adapters  
- Observabilidade integrada  
- Ferramentas avançadas  
- Comunidade consolidada  

---

# **Contribuindo**

Pull Requests e discussões são bem-vindas.  
Quanto mais simples e direta a contribuição, melhor.  
O Hypercore é construído de forma comunitária, transparente e pragmática.

---

# **Licença**
MIT.

---
