
# Resumo da estratégia

1. **Módulos**: persistence, queue/worker, agent-core (traits), workflow-lib, sessions, params, observability.
2. **Fase 0 (protótipo rápido)**: in-memory queue + sled persistence + tracing local.
3. **Fase 1 (durável)**: Redis Streams/Redis KV ou SQLite/SQLx para persistência + Prometheus + OpenTelemetry.
4. **Fase 2 (produção)**: HA Redis, backups, horizontal workers, monitoring dashboards.

# Tech stack recomendada (Rust)

* runtime: `tokio`
* serialização/validation: `serde`, `serde_json`, `schemars`, `validator`
* persistence (embedded KV): `sled` OR relational: `sqlx` + `sqlite/postgres`
* durable queue: `redis` (streams) or in-process `tokio::sync::mpsc` for dev
* observability: `tracing`, `tracing-subscriber`, `opentelemetry`, `metrics` + `prometheus` exporter
* telemetry/metrics: `metrics`, `metrics-exporter-prometheus`
* logging: `tracing`
* testing: `tokio::test`, `assert_json_diff` (ou serde asserts)
* packaging: workspace with crates: `hypercore-core`, `hypercore-persistence`, `hypercore-workflow`, `hypercore-agent`, `examples`

# Objetivos concretos e critérios de aceitação

* **Persistência**: Jobs, sessions e results persistidos em KV (sled) e exportáveis. CA: restart da app não perde jobs pendentes.
* **Paralelismo**: Workers escaláveis por processo; suporte a N concurrent workers; CA: processar 1000 jobs com concurrency configurável sem deadlocks.
* **Observabilidade**: métricas (jobs_processed_total, jobs_failed_total, worker_active), traces por job com trace-id. CA: Prometheus coleta métricas, traces mostram fluxo completo.
* **Workflow lib**: API `enqueue(job) -> job_id`, `worker.poll() -> Job`, `agent.handle(job) -> Result`. CA: exemplo de RAG pipeline executável.
* **Parâmetros estruturados**: JSON Schema + typed Rust structs + validação. CA: validação falha antes do enqueue.
* **Session mgmt**: session store com TTL e lookup por session_id; CA: autenticação via header session-id read/write e expiração.

---

# Plano em etapas (para IA gerar código)

## Etapa A — Estrutura do workspace (entregável)

Crie workspace com crates:

* `hypercore-core` (tipos, traits, errors)
* `hypercore-persistence` (sled/sqlx adapter)
* `hypercore-queue` (in-memory + redis streams adapter)
* `hypercore-agent` (Agent trait + example agent)
* `hypercore-workflow` (orquestrador fila→agente→resultado)
* `hypercore-session` (session store)
* `examples/mini_rag` (demo pipeline)

Critério: `cargo build` passa, testes unitários mínimos presentes.

### Prompt para IA:

"Crie um Cargo workspace com 6 crates: hypercore-core, hypercore-persistence, hypercore-queue, hypercore-agent, hypercore-workflow, hypercore-session. Para cada crate coloque um `lib.rs` básico e `Cargo.toml` com dependências tokio e serde. Gere um `README.md` curto."

## Etapa B — Definir tipos centrais (hypercore-core)

Entregável: tipos e traits.

* `JobId = Uuid`
* `SessionId = String`
* `struct Job { id: JobId, kind: String, params: serde_json::Value, created_at: DateTime<Utc>, session_id: Option<SessionId>, attempts: u32 }`
* `enum JobStatus { Pending, Processing, Done, Failed }`
* `trait Agent { async fn handle(&self, job: Job) -> Result<JobResult, AgentError>; fn name(&self) -> &str; }`
* `struct JobResult { job_id: JobId, output: serde_json::Value, succeeded: bool, metadata: Option<serde_json::Value> }`

Critério: tipos compilam e estão documentados.

### Prompt para IA:

"Gerar o arquivo lib.rs em hypercore-core contendo os tipos Job, JobResult, JobStatus, Agent trait async com tokio, erros com thiserror e UUIDs. Inclua derive serde onde aplicável."

## Etapa C — Persistence adapter (hypercore-persistence)

Entregável: trait `Persistence` e dois adapters:

* `trait Persistence { async fn save_job(&self, job: &Job) -> Result<(), Error>; async fn load_pending(&self) -> Result<Vec<Job>, Error>; async fn update_job_status(&self, id: JobId, status: JobStatus) -> Result<(), Error>; async fn save_result(&self, res: &JobResult) -> Result<(), Error>; }`
* Implementação 1: `sled` (KV) — rápido para protótipo.
* Implementação 2: `sqlx` + sqlite — optional for durable relational.

Critério: tests que inserem, read on restart simulated (create sled tree, write, read).

### Prompt para IA:

"Generate hypercore-persistence crate with Persistence trait and a sled-based implementation. Provide functions save_job, load_pending, update_job_status, save_result. Include unit tests that write a job and read it back."

## Etapa D — Queue & Worker (hypercore-queue)

Entregável:

* Abstração `Queue`:

  * `async fn enqueue(&self, job: Job) -> Result<JobId, Error>`
  * `async fn pop(&self) -> Result<Option<Job>, Error>`
  * `async fn ack(&self, job_id: JobId) -> Result<(), Error>`
* Implementations:

  * In-memory: `tokio::sync::mpsc` + visibility timeout logic
  * Redis Streams (durable) adapter (optional)
* Worker loop:

  * concurrency limit
  * exponential backoff for retries
  * poison queue on repeated failures

Critério: example runs 10 jobs concurrently.

### Prompt para IA:

"Generate hypercore-queue crate with Queue trait and an in-memory implementation using tokio::sync::mpsc. Implement enqueue/pop/ack and a Worker struct that spawns N tasks to process jobs by calling a supplied Agent trait object."

## Etapa E — Workflow lib (hypercore-workflow)

Entregável:

* High-level orchestrator:

  * `WorkflowEngine::start()` to spawn queue + workers + metrics.
  * `WorkflowEngine::submit(kind, params, session_id)` convenience API.
  * Support synchronous `await result` API (future that resolves when done) and fire-and-forget.
* Support chaining: allow agents to produce follow-up jobs.

Critério: example RAG where job→agent1 (embedding fetch) → agent2 (retrieval) → agent3 (response), with result returned.

### Prompt para IA:

"Generate hypercore-workflow crate. Implement WorkflowEngine which composes Queue + Persistence + Agents registry. Provide API submit_and_wait(job) that lets a caller await JobResult with timeout."

## Etapa F — Sessions (hypercore-session)

Entregável:

* Session store trait:

  * `create(session_data, ttl) -> SessionId`
  * `get(session_id) -> Option<SessionData>`
  * `extend(session_id, ttl)`
  * `delete(session_id)`
* Adapter: sled-based session store and Redis-based optional.
* Integration: attach session_id to jobs; workers pass session context to agents.

Critério: session created, persisted, and TTL respected (test uses fake clock or TTL shorter).

### Prompt para IA:

"Generate hypercore-session crate with a SessionStore trait and a sled-backed implementation. Include create/get/extend/delete and a unit test verifying TTL expiration."

## Etapa G — Parâmetros estruturados e validação (hypercore-core or hypercore-params)

Entregável:

* `ParameterSchema` using `schemars` to generate JSON Schema.
* A `Params` typed wrapper that validates input against a schema before enqueue.
* Helpers to derive schemas from Rust structs (derive macro optional).
* Provide built-in validators for types, ranges, required fields.

Critério: invalid params rejected with clear error before enqueue.

### Prompt para IA:

"Create a params module that provides ParamSchema using schemars and a function validate_params(schema: &schemars::schema::RootSchema, value: &serde_json::Value) -> Result<(), ValidationError>. Add example schema and test cases."

## Etapa H — Observability (integrated across crates)

Entregável:

* Use `tracing` spans for each job: span contains job_id, kind, session_id, attempt.
* Expose metrics via `metrics` crate: `jobs_enqueued_total`, `jobs_processed_total`, `jobs_failed_total`, `worker_active`.
* Prometheus exporter endpoint (simple HTTP server on /metrics) using `metrics-exporter-prometheus`.
* Optional: OpenTelemetry exporter for traces to Jaeger.

Critério: run example, curl `/metrics` returns metrics; traces include job_id.

### Prompt para IA:

"Add tracing spans to WorkflowEngine and Worker loops (job start/end/fail). Add metrics using metrics crate and expose a prometheus endpoint on port 9090. Provide integration test that asserts metrics increment."

## Etapa I — Example agents & end-to-end demo (examples/mini_rag)

Entregável:

* Minimal agents:

  * `EchoAgent` (returns params)
  * `SlowAgent` (simulates work, sleeps)
  * `RagAgent` (stub that calls pretend retriever)
* Example `main.rs` that:

  * Starts WorkflowEngine,
  * Registers agents,
  * Submits a job,
  * Awaits result,
  * Prints trace id and result.

Critério: `cargo run --example mini_rag` executes end-to-end.

### Prompt para IA:

"Create an example application mini_rag that starts the engine, registers EchoAgent and SlowAgent, submits a job and waits for the result. Print job_id and result JSON."

## Etapa J — Tests, CI, docs, and crate publish artifacts

Entregável:

* Unit tests for each crate.
* Integration tests exercising persistence across restart (sled).
* GitHub Actions: `cargo test`, clippy, fmt.
* README templates: API, run instructions, roadmap.

Critério: CI green on main branch.

### Prompt para IA:

"Create GitHub Actions workflow that runs `cargo build --workspace` and `cargo test`. Add basic README content describing how to run the example and how to switch adapters (sled -> redis)."

---

# Design patterns & implementation details (assinaturas e snippets rápidos)

## Agent trait (hypercore-core)

```rust
#[async_trait::async_trait]
pub trait Agent: Send + Sync + 'static {
    fn name(&self) -> &str;
    async fn handle(&self, ctx: AgentContext, job: Job) -> Result<JobResult, AgentError>;
}
```

`AgentContext` contains tracing span, session data accessor, persistence handle, optional http client.

## WorkflowEngine API (hypercore-workflow)

```rust
pub struct WorkflowEngine { /* ... */ }

impl WorkflowEngine {
    pub async fn new(cfg: EngineConfig) -> Self;
    pub async fn register_agent(&self, agent: Arc<dyn Agent>);
    pub async fn submit(&self, kind: &str, params: serde_json::Value, session: Option<SessionId>) -> Result<JobId, Error>;
    pub async fn submit_and_wait(&self, kinda: &str, params: serde_json::Value, timeout: Duration) -> Result<JobResult, Error>;
    pub async fn start(&self) -> Result<(), Error>;
    pub async fn shutdown(&self) -> Result<(), Error>;
}
```

## Persistence trait

```rust
#[async_trait::async_trait]
pub trait Persistence: Send + Sync {
    async fn save_job(&self, job: &Job) -> Result<(), PersistenceError>;
    async fn fetch_next(&self) -> Result<Option<Job>, PersistenceError>;
    async fn update_status(&self, id: JobId, status: JobStatus) -> Result<(), PersistenceError>;
    async fn save_result(&self, result: &JobResult) -> Result<(), PersistenceError>;
}
```

## Queue trait (simpler)

```rust
#[async_trait::async_trait]
pub trait Queue: Send + Sync {
    async fn enqueue(&self, job: Job) -> Result<JobId, QueueError>;
    async fn reserve(&self, timeout: Duration) -> Result<Option<Job>, QueueError>;
    async fn complete(&self, job_id: JobId) -> Result<(), QueueError>;
    async fn fail(&self, job_id: JobId, err: &str) -> Result<(), QueueError>;
}
```

---

# Test cases mínimos (aceitação)

1. Enqueue job -> worker processes -> result stored -> metrics increment.
2. Restart process (simulate by re-instantiating persistence) -> pending jobs still present.
3. Invalid params rejected.
4. Session TTL expires; new lookup returns none.
5. Concurrent workers (N=10) process M=100 jobs; all succeed with no panics.

---

# Deploy / local infra quick recipe (docker-compose)

* sled requires nothing.
* For Redis + Prometheus:

  * redis: latest
  * prometheus: with simple config scraping `http://hypercore:9090/metrics`
  * jaeger: optional for traces

Include a `docker-compose.yaml` stub generator in repo.

### Prompt for IA:

"Generate a docker-compose.yml that includes redis, prometheus, and jaeger services with basic configuration for local testing."

---

# Prompts prontos para geração de código (copiar/colar para seu generator)

Vou dar 8 prompts que você pode alimentar em sequência na IA para gerar o código automatizado. Cada prompt deve ser usado por crate/feature — a IA produzirá o código completo, testes e README.

1. **Workspace scaffold**

   ```
   Create a Rust workspace with crates: hypercore-core, hypercore-persistence, hypercore-queue, hypercore-agent, hypercore-workflow, hypercore-session. Each crate must have Cargo.toml, lib.rs and minimal dependencies (tokio, serde). Output file tree and minimal files.
   ```

2. **Core types & agent trait**

   ```
   Implement hypercore-core: Job, JobResult, JobStatus, Agent trait (async), AgentContext, Error types. Use uuid, chrono, serde, thiserror. Provide examples and unit tests for serialization.
   ```

3. **Sled persistence**

   ```
   Implement hypercore-persistence sled adapter that implements Persistence trait: save_job/load_pending/update_status/save_result. Include tests that write job, read pending, update status, and store result.
   ```

4. **In-memory queue & worker**

   ```
   Implement hypercore-queue in-memory Queue using tokio::sync::mpsc with reserve/ack/fail semantics and visibility timeout. Provide Worker struct that processes using a supplied Agent. Add tests for concurrency.
   ```

5. **WorkflowEngine orchestration**

   ```
   Implement hypercore-workflow: WorkflowEngine that composes Persistence + Queue, agent registry, submit_and_wait API, start/shutdown. Integrate tracing spans and metrics counters in critical paths.
   ```

6. **Session store**

   ```
   Implement hypercore-session with SessionStore trait and sled-backed implementation. Provide create/get/extend/delete and TTL logic. Add unit tests for expiration.
   ```

7. **Params validation**

   ```
   Implement a params module using schemars for schema generation and a validate_params function that accepts a schema and a serde_json::Value and returns ValidationErrors. Add example schema file and tests.
   ```

8. **Example app**

   ```
   Generate examples/mini_rag binary that starts engine, registers two agents (EchoAgent and SlowAgent), creates a session, submits a job and waits for result, prints metrics endpoint and job result.
   ```

---

# Riscos, trade-offs e recomendações rápidas

* **Trade-off sled vs SQL**: sled é simples e fast for prototyping, mas difficult for complex queries and migrations. Use sled for MVP; migrate to Postgres/sqlx when you need joins or analytics.
* **Queue durability**: in-memory is fine for dev. For production use Redis Streams (or Kafka) for persistence and consumer groups.
* **Observability**: start with `tracing` + `metrics` locally; add Jaeger/OTel later.
* **Security**: session management must include authentication and signing if used across clients. For now session-id opaque and stored server-side is fine.
* **Backward compatibility**: design Job schema versioning (`version` field).

---

# Próximo passo imediato (o que eu gero agora se você quiser)

Diga qual crate quer que eu gere primeiro (core, persistence sled, queue in-memory, workflow engine, session, params, example). Eu gero o código do crate escolhido com testes e instruções para executar localmente. Indique *um* alvo e eu entrego o artefato completo.

Quer que eu comece gerando o `hypercore-core` agora?
