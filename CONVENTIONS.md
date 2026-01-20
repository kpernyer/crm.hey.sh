# CRM.HEY.SH Development Conventions

## Rust Development Standards

### 1. Rust Edition, Toolchain & Baseline
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

### 2. Architectural Patterns
#### 2.1 Axum Framework (Default API Framework)
- Modern async stack built on **Tower**.
- Strong typing + ergonomic routing.
- Best ecosystem alignment for: middlewares, extractors, OpenAPI generation, testing.
- **Opinion**: **Axum is the default API framework** for all internal & external services.

#### 2.2 Tokio Runtime
- Industry-standard async executor.
- Mature ecosystem, strong performance.
- Required by Axum, Tonic, many async crates.
- **Opinion**: **Tokio is the only runtime allowed.**
  No mixed async runtimes.

### 3. Layered Architecture (Clean Architecture)
The project follows a clean architecture pattern with distinct layers:

#### 3.1 Handlers Layer
- HTTP request handlers located in `src/handlers/`
- Thin layer that extracts/validates inputs and maps responses
- No business logic allowed in handlers
- Convert domain errors to HTTP responses

#### 3.2 Services Layer
- Business orchestration logic in `src/services/`
- Coordinate between domain entities and repositories
- Handle cross-cutting business concerns
- Use dependency injection for repositories and other services

#### 3.3 Domain Layer
- Pure business logic in `src/domain/`
- Contains entities, value objects, and domain services
- No I/O operations (no database calls, no external API calls)
- No dependencies on infrastructure concerns

#### 3.4 Models Layer
- Data Transfer Objects (DTOs) in `src/models/`
- Request/response structures for API endpoints
- Validation attributes and serialization options

#### 3.5 Repositories Layer
- Data access logic in `src/repositories/`
- Database queries and persistence concerns
- Return domain objects to upper layers
- Abstract database technology from business logic

#### 3.6 AI/LLM Tools Layer
- AI integration and LLM-powered tools in `src/ai/` and `src/llm_tools/`
- Isolated from core business logic
- Pluggable components that can be extended

### 4. Data Layer Standards
#### 4.1 SurrealDB (Primary System-of-Record)
- Multi-model: relational + document + graph.
- SQL-like queries + embedded mode + distributed mode.
- Clean Rust client + lightweight footprint.
- **Opinion**: **SurrealDB is the single primary database unless a domain requires otherwise.**
- **Schema & migrations**:
  - Keep schema/migrations in a dedicated module (e.g., `db/schema.rs`).
  - Prefer idempotent SurrealQL `DEFINE` blocks; migrations should be safe to run repeatedly.
  - In dev, run migrations on startup; in prod, run via an explicit admin/ops command or controlled startup step.

#### 4.2 Configuration Management
- Use **config crate** + layered approach:
  1. base.yaml
  2. env-specific.yaml
  3. environment variables
  4. secrets store overrides

### 5. API Design Standards
#### 5.1 REST/OpenAPI for External APIs
- Use Axum + utoipa (OpenAPI generator).
- Provide scripts for:
  - cURL
  - HTTPie
  - Postman collection
  - TypeScript client generation
- **Opinion**: **Every service must publish an auto-generated OpenAPI spec.**

#### 5.2 Error Handling (Strict)
- No `unwrap`, `expect`, or `panic!` in production paths (libraries, services, servers).
- Use `thiserror` for clean error types.
- Use `anyhow` only at boundaries (CLI, handlers, tests).
- **No `anyhow` in libraries or domain layers**:
  - `src/lib.rs` modules, services, and repositories must return typed errors.
  - Define domain errors with `thiserror` and use `Result<T, DomainError>`.
  - Convert domain errors to transport errors (HTTP/gRPC) in handlers, then optionally wrap with `anyhow` at bin boundaries.
- Prefer domain-specific error types inside services.

### 6. Code Style & Engineering Habits
- Always enable clippy & fmt checks:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  cargo fmt --all -- --check
  ```

- **Testing Strategy**:
  - Unit tests: Required for every module. Use `proptest` for generative property tests.
  - Integration tests: Use `cargo test -- --ignored` for tests with external services.
  - API contract tests: Auto-verify OpenAPI definitions.

### 7. Idiomatic Rust Production Code
This section defines strict, idiomatic Rust rules for production code. These are enforceable via review and Clippy.

#### 7.1 Immutability & Ownership
- Immutable by default: use `let`, not `let mut`, unless mutation is required.
- Prefer iterator transforms over mutating loops.
- Avoid unnecessary `clone()`; justify clones at ownership boundaries or for explicit performance tradeoffs.
- Use `Arc` only for cross-task/thread shared ownership; otherwise pass references.
- Avoid global state/singletons; use explicit dependency injection.

#### 7.2 Data Modeling
- Use newtypes for IDs instead of raw `String`/`i64` (e.g., `struct ContactId(String);`).
- Prefer enums over boolean flags or magic strings.
- Keep structs focused; avoid "god structs" with many `Option<T>` fields unless modeling sparse data.
- Derive traits intentionally (don't cargo-cult `Clone`, `Debug`, `Serialize`).
- Use `#[non_exhaustive]` for public enums expected to grow.

#### 7.3 Async & Concurrency
- Never hold locks across `.await`.
- Use `tokio::sync` primitives in async code; avoid std mutexes.
- Spawn tasks only when concurrency is required; keep flow structured.
- Add timeouts around external calls (`tokio::time::timeout` or Tower timeouts).

#### 7.4 Avoiding Bad Patterns
- No business logic in handlers; handlers extract/validate/map only.
- No scattered DB queries; repositories own data access.
- Parameterize DB queries; avoid string interpolation.
- Avoid `cfg!(test)` branches in production code; use traits/mocks.

### 8. LLM Integration Conventions
- Use **OpenRouter** as the default LLM API aggregator.
- Maintain keys in secrets manager, never in code.
- **Opinion**: **All LLM integrations go through a single `llm_client` crate.**

### 9. Observability Standards
- Mandatory:
  - OpenTelemetry traces
  - Prometheus metrics
  - Structured logging (`tracing`)
- **Operational tracing rules**:
  - Every request creates a root span in the handler.
  - Every public service/repository method must run inside a span (`#[tracing::instrument]` or manual `span!`).
  - Add structured fields for domain identifiers (e.g., `contact_id`, `campaign_id`, `company_id`).
  - Create child spans for DB queries and external calls.

### 10. Build & Development Tools
#### 10.1 Justfile Standardization
Every component must have a `Justfile` with:
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

# Component-specific targets
dev:
    # Development server command
```

#### 10.2 Dependency Management
- Use Cargo for Rust dependencies
- Pin major versions for stability
- Regular updates through dependabot or manual review
- Separate dev dependencies from production

### 11. File Naming and Organization
- Use snake_case for module names and files
- Keep related functionality in the same directory
- Follow the established directory structure:
  ```
  src/
  ├── handlers/     # HTTP route handlers
  ├── models/       # Data models
  ├── services/     # Business logic
  ├── domain/       # Pure business logic
  ├── repositories/ # Data access
  ├── ai/          # AI integration
  └── llm_tools/   # LLM-powered tools
  ```

### 12. Documentation Standards
- Inline documentation for public APIs using `///`
- Module-level documentation explaining purpose
- README files for each major component
- Architecture Decision Records (ADRs) for significant design choices
- Use consistent formatting and examples