# HYPERCORE: Framework Neuro-Simbólico
### Motor de Raciocínio Híbrido em Rust – Seguro, Rápido e Determinístico

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

## **O que é o HyperCore?**

O **HyperCore** é um *framework de raciocínio híbrido* (Neuro-Symbolic), projetado para sistemas que precisam de:

- **Flexibilidade Neural**: Interpretação de linguagem e contexto via LLMs.
- **Precisão Simbólica**: Regras lógicas, fatos estruturados e dedução determinística.
- **Confiabilidade**: Traceabilidade completa das decisões e estado.
- **Performance**: Execução concorrente e segura em Rust.

Ao contrário de runtimes de "agentes" tradicionais que dependem exclusivamente de prompts, o HyperCore fundamenta o raciocínio em uma **memória estruturada de fatos e regras**, utilizando LLMs como *motores de percepção* e algoritmos simbólicos como *motores de dedução*.

---

## **Arquitetura**

O workspace é composto por crates modularizados:

### 1. **`hypercore`** (Core)
O orquestrador central. Gerencia sessões, coordena os motores de raciocínio e mantém o ciclo de vida da execução.

### 2. **`hypercore-symbolic`** (Symbolic Engine)
Motor de dedução lógica. Gerencia fatos (Entities-Attributes-Values), regras e inferência (forward-chaining). Garante consistência lógica.

### 3. **`hypercore-hybrid`** (Planner)
O "cérebro" do sistema. Planeja passos de raciocínio, decidindo quando consultar o modelo neural e quando aplicar deduções lógicas.

### 4. **`hypercore-openai`** (Neural Layer)
Interface neural. Transforma texto não estruturado em fatos estruturados (JSON) e propostas de raciocínio.

### 5. **`hypercore-memory-sqlite`** (Persistent Memory)
Persistência transacional de fatos, regras e traces de raciocínio.

### 6. **`hypercore-tools`** (Utilities)
Ferramentas de suporte e conexões externas.

---

## **Instalação**

```toml
[dependencies]
hypercore = { path = "crates/hypercore" }
hypercore-symbolic = { path = "crates/hypercore-symbolic" }
hypercore-hybrid = { path = "crates/hypercore-hybrid" }
```

---

## **Conceito de Uso**

```rust
use hypercore::Orchestrator;
use hypercore_hybrid::HybridPlanner;

#[tokio::main]
async fn main() {
    // 1. Inicializa o Orquestrador
    let mut orchestrator = Orchestrator::new();

    // 2. Define um objetivo
    let goal = "Validar contrato bancário sob regras de 2024";

    // 3. Executa o ciclo híbrido
    let result = orchestrator.reason(goal).await.unwrap();

    println!("Resultado: {}", result.conclusion);
    println!("Trace: {:?}", result.trace);
}
```

---

## **Roadmap (Transição)**

### **v0.1 – Fundação (Atual)**
- [x] Estrutura de crates definida.
- [x] Motor Simbólico básico (Fatos e Regras).
- [x] Persistência SQLite inicial.
- [x] Remoção de abstrações legadas de "Agentes".

### **v0.2 – Ciclo Híbrido**
- [ ] Integração completa Neural -> Simbólica.
- [ ] Parser robusto de JSON para Fatos.
- [ ] Planner capaz de loops de refinação.

### **v0.3 – Observabilidade e Tools**
- [ ] Tracing completo de cada passo de dedução.
- [ ] Ferramentas de inspeção de estado.

---

## **Licença**

MIT.
