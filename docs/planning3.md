Direto e objetivo: vou montar o **markdown oficial da transição do HyperCore para um framework neuro-simbólico**, considerando **exatamente os crates que você já tem**:

* `hypercore` (runtime / orchestrator)
* `hypercore-memory-sqlite` (persistência)
* `hypercore-openai` (interface neural)
* `hypercore-tools` (extensões e utilidades)

E vou te entregar um documento **pronto para o repositório**, claro, direto e sem enrolação.

---

# **HyperCore: Transição para Framework Neuro-Simbólico**

*(Proposta de Arquitetura e Evolução – v0.1)*

O HyperCore já possui um runtime modular, memória estruturada, integração neural via OpenAI e um ecossistema de ferramentas.
Este documento define o plano de transição para transformar o HyperCore em um **framework de raciocínio híbrido (neuro-simbólico)**.

---

## 1. Objetivo da Transição

O HyperCore deixa de ser apenas um orquestrador de workflows para assumir o papel de **motor cognitivo híbrido**, combinando:

1. **Raciocínio neurais** (LLMs via `hypercore-openai`)
2. **Raciocínio simbólico** (novo crate)
3. **Planejamento híbrido** (neural + simbólico)
4. **Memória persistente estruturada** (via `hypercore-memory-sqlite`)
5. **Execução observável e auditável** (via runtime do `hypercore`)

O foco é criar um framework capaz de:

* Interpretar problemas (neural)
* Manipular fatos e regras (simbólico)
* Planejar passos de raciocínio (híbrido)
* Explicar decisões (observabilidade)
* Executar em escala (concorrência do Rust)

---

## 2. Crates Existentes e Suas Novas Funções

### **2.1. `hypercore` (já existente)**

Passa a ser o **núcleo operacional** do framework.

Novas responsabilidades:

* Orquestrar passos de raciocínio (jobs)
* Coordenar unidades neurais e simbólicas
* Gerenciar sessões de raciocínio
* Registrar explicações e traces
* Controlar paralelismo de deduções

Nada precisa ser reescrito. Apenas ampliado com novos tipos de "ReasoningTasks".

---

### **2.2. `hypercore-memory-sqlite` (já existente)**

Evoluir para a **Memória Neuro-Simbólica do HyperCore**:

Novas entidades:

* `facts` (fatos simbólicos)
* `rules` (regras, constraints)
* `symbolic_state` (estado lógico de cada sessão)
* `reasoning_trace` (passos do raciocínio)

Funções adicionais:

* Versionamento de estado lógico
* Query semântica + query simbólica
* Integração com embeddings se necessário (futuro)

---

### **2.3. `hypercore-openai` (já existente)**

Esse crate vira a **Camada Neural do HyperCore**.

Novas responsabilidades:

* Produzir *símbolos estruturados* (JSON estrito)
* Validar saída neural contra regras
* Implementar modos: “interpret”, “propose”, “refine”
* Suporte ao ciclo neural → simbólico → neural

O papel dele agora é entregar **informação estruturada**, não texto solto.

---

### **2.4. `hypercore-tools` (já existente)**

Vira um conjunto de utilidades para:

* conversão entre texto ↔ símbolos
* libraries padronizadas (ex.: filtros, normalização, schema enforcement)
* builders para fatos, regras e queries simbólicas
* validações lógicas simples

Ele serve de cola entre o neural, o simbólico e o runtime.

---

## 3. Novos Crates (Expansão do HyperCore)

### **3.1. `hypercore-symbolic` (novo)**

O coração simbólico:

* Representação de fatos e regras
* Motor de dedução
* Integração com e-graphs (`egg`)
* Constraints / matching
* Verificador lógico
* Reescrita simbólica
* Execução determinística

Esse crate cria o lado “clássico” do raciocínio.

---

### **3.2. `hypercore-hybrid` (novo)**

O **planner neuro-simbólico**:

Funções centrais:

* Decidir entre neural / simbólico / ambos
* Encadear passos de raciocínio
* Corrigir inconsistências neurais com regras
* Especificar pipelines lógicos
* Compor raciocínio distribuído

É aqui que a inteligência do sistema realmente emerge.

---

## 4. Fluxo de Raciocínio Híbrido (High-Level)

1. Usuário envia uma query.
2. O `hypercore` cria uma sessão e inicializa o estado simbólico.
3. O `hypercore-hybrid` decide o primeiro passo:

   * interpretação neural (via `hypercore-openai`)
   * dedução simbólica (via `hypercore-symbolic`)
4. O resultado é salvo na memória (sqlite).
5. O planner executa iterações até convergência.
6. A resposta final é montada e entregue.
7. Toda a cadeia fica registrada no trace.

---

## 5. Estrutura Proposta do Workspace

```plaintext
hypercore/
├─ hypercore/                     # runtime + executor
├─ hypercore-memory-sqlite/       # memória persistente
├─ hypercore-openai/              # camada neural
├─ hypercore-tools/               # utilidades transversais
├─ hypercore-symbolic/            # motor simbólico (novo)
├─ hypercore-hybrid/              # planner híbrido (novo)
└─ examples/
   ├─ legal_reasoning/
   ├─ troubleshooting/
   └─ medical_triage/
```

---

## 6. MVP da Transição (v0.1 → v0.2)

### v0.1.1 – Preparação

* Normalizar APIs
* Padronizar `SessionId`, `Fact`, `Rule`, `Symbol`
* Integrar memory + runtime

### v0.1.2 – Núcleo simbólico mínimo

* facts
* regras simples
* dedução básica (IF-THEN)

### v0.1.3 – Primeira pipeline híbrida

* neural interpreta
* simbólico valida
* persistência da sessão

### v0.2 – HyperCore Neuro-Symbolic Engine

* planner híbrido
* tracing completo
* exemplos reais
* documentação oficial

---

## 7. Próximo passo obrigatório

Precisamos definir **as estruturas centrais**:

* `Fact`
* `Rule`
* `SymbolicState`
* `NeuralOutput`
* `ReasoningStep`
* `HybridPlan`

Quer que eu monte agora o **arquivo `SYMBOLIC.md`** com as definições formais desses tipos e o esqueleto do motor simbólico? Ou prefere começar pelo **planner híbrido**?
