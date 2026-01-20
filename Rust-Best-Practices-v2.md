# Rust Best Practices (Opinionated for Modern, Production-Grade Systems)

## 1. Rust Edition, Toolchain & Baseline
- **Minimum Rust version**: Always assume **Rust 1.76+** as baseline, but target **Rust Edition 2024** for all new services.
- Do **not downgrade** toolchain because of external crates that lag; instead:
  - Prefer actively maintained crates.
  - Fork or replace stale dependencies.
- **Edition policy**:
  - All new crates/binaries must use **Edition 2024**.
  - Existing Edition 2021 workspaces should migrate when dependencies allow; track blockers in README or a short ADR.
- Enforce via:
  ```toml
  [package]
  edition = "2024"

  [workspace.metadata.rust-version]
  rust-version = "1.76"
  ```

## 2. Architectural Opinions
### 2.1 Axum (Why?)
- Modern async stack built on **Tower**.
- Strong typing + ergonomic routing.
- Best ecosystem alignment for: middlewares, extractors, OpenAPI generation, testing.
- Opinion: **Axum is the default API framework** for all internal & external services.

### 2.2 Tokio Runtime
- Industry-standard async executor.
- Mature ecosystem, strong performance.
- Required by Axum, Tonic, many async crates.
- Opinion: **Tokio is the only runtime allowed.**  
  No mixed async runtimes.

## 3. Data Layer
### 3.1 SurrealDB (Primary System-of-Record)
- Multi-model: relational + document + graph.
- SQL-like queries + embedded mode + distributed mode.
- Clean Rust client + lightweight footprint.
- Opinion: **SurrealDB is the single primary database unless a domain requires otherwise.**
- **Schema & migrations**:
  - Keep schema/migrations in a dedicated module (e.g., `db/schema.rs`).
  - Prefer idempotent SurrealQL `DEFINE` blocks; migrations should be safe to run repeatedly.
  - In dev, run migrations on startup; in prod, run via an explicit admin/ops command or controlled startup step.

### 3.2 Vector Storage
- Use **Qdrant** (server) and **LanceDB** (local/embedded).
- Pure Rust clients, high performance, simple integration.
- Opinion:
  - **Qdrant** for production vector search.
  - **LanceDB** for local, serverless or embedded vector workflows.

### 3.3 Search & Analytics
- **Apache Arrow + DataFusion** for analytics.
- Opinion: Use DataFusion for all query-heavy internal analytics workloads.

## 4. API Protocols
### 4.1 gRPC with Tonic
- For internal service-to-service communication.
- Strong typing, streaming, codegen, performance.
- Opinion: **gRPC/Tonic is mandatory for intra-service communication.**

### 4.2 REST/OpenAPI for External APIs
- Use Axum + utoipa (OpenAPI generator).
- Provide scripts for:
  - cURL
  - HTTPie
  - Postman collection
  - TypeScript client generation
- Opinion: **Every service must publish an auto-generated OpenAPI spec.**

## 5. Messaging & Eventing
- Preferred: **NATS** (simple, fast, Rust-friendly).
- Supports JetStream for durable messages.
- Opinion: **Default message bus is NATS.**

## 6. Workflow Engines
- Preferred: **Temporal** (Rust SDK, strong guarantees).
- Opinion: Temporal handles:
  - long-running workflows
  - retries
  - shipment/order lifecycle
  - distributed coordination

## 7. Secrets & Config
### 7.1 Config Management
- Use **config crate** + layered approach:
  1. base.yaml  
  2. env-specific.yaml  
  3. environment variables  
  4. secrets store overrides  

### 7.2 Secret Storage
- No plain `.env` files for production.
- Opinion: Use:
  - Google Secret Manager (if on GCP)
  - Vault (if self-hosting)

### 7.3 LLM Key Strategy
- Use **OpenRouter** as the default LLM API aggregator.
- Maintain keys in secrets manager, never in code.
- Opinion: **All LLM integrations go through a single `llm_client` crate.**

## 8. Agent Frameworks
- Evaluate:
  - **MCP** for tool-exposing services.
  - **A2A** for inter-agent protocols.
  - LangGraph (Rust ports still emerging).
- Opinion: 
  - Support MCP for tool endpoints.
  - Adopt A2A once stable in Rust.
  - Keep agents modular and language-agnostic.

## 9. Testing Strategy
### 9.1 Unit Tests
- Required for every module.
- Use `proptest` for generative property tests.

### 9.2 Integration Tests
- Use `cargo test -- --ignored` for tests with external services.
- Standup ephemeral SurrealDB + Qdrant.

### 9.3 API Contract Tests
- Auto-verify OpenAPI definitions.

## 10. Code Style & Engineering Habits
- Always enable clippy & fmt checks:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  cargo fmt --all -- --check
  ```

- Use `thiserror` for clean error types.
- Use `anyhow` only at boundaries (CLI, handlers, tests).
- **No `anyhow` in libraries or domain layers**:
  - `src/lib.rs` modules, services, and repositories must return typed errors.
  - Define domain errors with `thiserror` and use `Result<T, DomainError>`.
  - Convert domain errors to transport errors (HTTP/gRPC) in handlers, then optionally wrap with `anyhow` at bin boundaries.
- Prefer domain-specific error types inside services.

## 11. Rust Code Quality (Idiomatic, Production)
This section defines strict, idiomatic Rust rules for production code. These are enforceable via review and Clippy.

### 11.1 Error Handling (Strict)
- No `unwrap`, `expect`, or `panic!` in production paths (libraries, services, servers).
- Use `thiserror` for domain errors; one error enum per bounded context/module.
- Prefer `Result<T, DomainError>` internally. Map only at boundaries:
  - Axum handlers: convert to HTTP responses via `IntoResponse`.
  - Tonic handlers: convert to gRPC `Status`.
  - Binaries/CLIs: may return `anyhow::Result` at `main`.
- Add context at boundaries (`anyhow::Context` or explicit error variants), not deep inside core logic.
- Errors must be structured and matchable; avoid `String`ly‑typed errors.

### 11.2 Immutability & Ownership
- Immutable by default: use `let`, not `let mut`, unless mutation is required.
- Prefer iterator transforms over mutating loops.
- Avoid unnecessary `clone()`; justify clones at ownership boundaries or for explicit performance tradeoffs.
- Use `Arc` only for cross‑task/thread shared ownership; otherwise pass references.
- Avoid global state/singletons; use explicit dependency injection.

### 11.3 Data Modeling
- Use newtypes for IDs instead of raw `String`/`i64` (e.g., `struct ShipmentId(String);`).
- Prefer enums over boolean flags or magic strings.
- Keep structs focused; avoid “god structs” with many `Option<T>` fields unless modeling sparse data.
- Derive traits intentionally (don’t cargo‑cult `Clone`, `Debug`, `Serialize`).
- Use `#[non_exhaustive]` for public enums expected to grow.

### 11.4 Async & Concurrency
- Never hold locks across `.await`.
- Use `tokio::sync` primitives in async code; avoid std mutexes.
- Spawn tasks only when concurrency is required; keep flow structured.
- Add timeouts around external calls (`tokio::time::timeout` or Tower timeouts).

### 11.5 Avoiding Bad Patterns
- No business logic in handlers; handlers extract/validate/map only.
- No scattered DB queries; repositories own data access.
- Parameterize DB queries; avoid string interpolation.
- Avoid `cfg!(test)` branches in production code; use traits/mocks.

## 12. Using LLMs for Rust (House Style)
Given a modern, high‑capability model, use it as a senior Rust collaborator. Be explicit about constraints and ask for production‑grade outcomes.

### 11.1 Always Request Production‑Grade Output
- Ask for **"opinionated, production‑grade"** solutions.
- Specify hard constraints in the prompt:
  - no `unwrap`, no `expect`, no `panic!`
  - structured, domain‑level error types
  - `tracing` everywhere (spans, events, structured fields)
  - Axum/Tonic + Tokio stack
  - Rust 2024 edition assumptions
- Example prompt:
  > "Refactor this into an opinionated, production‑grade Rust 2024 service: no unwrap/panic, structured errors, tracing, Axum/Tonic." 

### 11.2 Prefer Refactoring Over Greenfield
- Use the model to improve existing code, not just generate new code.
- Typical asks:
  - "Refactor this Axum handler into layers: handler → service → repository."
  - "Introduce domain errors + map to HTTP/gRPC status codes."
  - "Add tracing spans per request and per DB call."
  - "Make this module testable with traits and mocks." 

### 11.3 Let It Propose Architecture, Then Review
- Ask the model to propose module boundaries, traits, and data types.
- Treat proposals as a draft architecture you review and adjust.
- Example prompt:
  > "Design the trait + module layout for this feature. Optimize for testability, layering, and clean domain types." 

### 11.4 Promote Good Patterns Into This Document
- When an output matches house style (errors, FFI wrappers, Tonic layout, etc.), copy it here.
- Then tell the model to follow that style in future work.
- Example prompt:
  > "Follow the patterns in Rust‑Best‑Practices‑v1.md for errors and layering." 

## 13. Deployment
### 13.1 Terraform
- Terraform is default for all infra.
- No YAML hell from cloud vendors.
- All services must have:
  - main.tf
  - variables.tf
  - outputs.tf

### 13.2 Justfile
Opinion: Every repo **must** have a `Justfile` with:
```makefile
default: fmt lint test

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

test:
    cargo test

run:
    cargo run

build:
    cargo build --release

deploy:
    terraform apply
```

## 14. Observability
- Mandatory:
  - OpenTelemetry traces
  - Prometheus metrics
  - Structured logging (`tracing`)
- **Operational tracing rules**:
  - Every request creates a root span in the handler.
  - Every public service/repository method must run inside a span (`#[tracing::instrument]` or manual `span!`).
  - Add structured fields for domain identifiers (e.g., `shipment_id`, `lane_id`, `carrier_id`).
  - Create child spans for DB queries and external calls.

## 15. Microservice Design Checklist
- Clear bounded context
- gRPC internally, REST externally
- Shared proto repo
  - SurrealDB schemas well-defined
- Versioned APIs
- Health/ready endpoints
- Automated load tests before deploy

---

## Conclusion
This document defines an **opinionated, modern Rust engineering standard**—centered on Axum, Tokio, SurrealDB, Qdrant/LanceDB, gRPC, NATS, Temporal, Terraform, robust testing, strong typing, observability, and clean API definitions.

Rust 2024 + async-first architecture ensures long-term maintainability, reliability, performance, and ecosystem alignment.
