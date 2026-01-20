# LLM Integration in CRM.HEY.SH

## LLM Strategy and Architecture

### 1. Core LLM Philosophy
The CRM.HEY.SH platform integrates Large Language Models following these guiding principles:

- **OpenRouter as Default**: Use OpenRouter as the primary LLM API aggregator to support flexibility across different providers
- **Centralized Client**: All LLM integrations go through a single `llm_client` crate for consistent handling
- **Modular Tools**: LLM capabilities are exposed as pluggable tools in the `src/llm_tools/` directory
- **Production-Grade Integration**: No `unwrap`/panic, structured errors, tracing, and proper async handling in all LLM-related code

### 2. Current LLM Tools Implementation

The system includes several LLM-powered tools located in the `src/llm_tools/` directory:

#### 2.1 Email Content Generator (`email_generator.rs`)
- Generates personalized email content for marketing campaigns
- Integrates with the campaign execution system
- Uses structured prompts to maintain brand voice and compliance
- Supports variable templating for dynamic content

#### 2.2 Social Media Content Generator (`social_post_generator.rs`)
- Creates social media content for campaigns
- Adapts content to different platforms (Twitter, LinkedIn, Facebook)
- Maintains consistent messaging across channels
- Implements appropriate character limits and formatting

#### 2.3 Landing Page Generator (`landing_page_generator.rs`)
- Builds custom landing pages dynamically
- Generates HTML/CSS from campaign specifications
- Optimizes for conversion and user engagement
- Includes responsive design principles

### 3. LLM Integration Patterns

#### 3.1 Tool-First Approach
- Each LLM capability is encapsulated as a dedicated tool
- Tools follow consistent interfaces and error handling
- Easy to extend with new LLM capabilities
- Testable independently from core business logic

#### 3.2 Prompt Engineering Practices
- Structured, template-based prompts for consistency
- Parameterized prompts that accept context from the business domain
- System messages that define role, constraints, and expected output format
- Input validation before sending to LLM (sanitize and validate user data)

#### 3.3 Response Processing
- Structured parsing of LLM outputs
- Validation of generated content against business rules
- Fallback mechanisms for when LLM responses don't meet criteria
- Caching of expensive generations when appropriate

### 4. Using LLMs for Rust Development (House Style)

Given a modern, high-capability model, our approach is to use it as a senior Rust collaborator. Be explicit about constraints and ask for production-grade outcomes.

#### 4.1 Production-Grade Output Requests
When working with LLMs for code generation, we request:
- **"Opinionated, production-grade"** solutions
- Hard constraints: no `unwrap`, no `expect`, no `panic!`
- Structured, domain-level error types
- `tracing` everywhere (spans, events, structured fields)
- Axum/Tonic + Tokio stack
- Rust 2024 edition assumptions

Example prompt:
> "Refactor this into an opinionated, production-grade Rust 2024 service: no unwrap/panic, structured errors, tracing, Axum/Tonic."

#### 4.2 Refactoring Over Greenfield
Our typical approach with LLMs:
- "Refactor this Axum handler into layers: handler → service → repository."
- "Introduce domain errors + map to HTTP/gRPC status codes."
- "Add tracing spans per request and per DB call."
- "Make this module testable with traits and mocks."

#### 4.3 Architecture Proposals
Ask the model to propose module boundaries, traits, and data types, then review the proposal:
> "Design the trait + module layout for this feature. Optimize for testability, layering, and clean domain types."

### 5. Security and Privacy Considerations

#### 5.1 Data Sanitization
- Personal data is sanitized before being sent to LLMs
- PII is redacted from prompts where possible
- Context windows are limited to minimize data exposure
- All sensitive operations are subject to audit trails

#### 5.2 API Key Management
- Keys stored in secrets manager, never in code
- Rotation policies for LLM API keys
- Rate limiting and monitoring of LLM usage
- Separate keys for different environments

### 6. Performance and Cost Optimization

#### 6.1 Caching Strategies
- Cache deterministic generations to reduce costs
- Implement smart invalidation based on prompt changes
- Store generated content with metadata for reuse
- Pre-generate common templates during low-usage periods

#### 6.2 Model Selection
- Choose appropriate models based on task complexity
- Use faster, cheaper models for simple transformations
- Reserve more expensive models for creative tasks
- Implement fallbacks to simpler models if quota is exceeded

#### 6.3 Batch Processing
- Group similar requests to optimize API usage
- Implement queue-based processing for non-real-time tasks
- Use bulk endpoints when available from providers
- Optimize token usage through efficient prompt design

### 7. Monitoring and Observability

#### 7.1 LLM-Specific Metrics
- Token usage tracking per feature/function
- Response times for different LLM operations
- Success/failure rates of generations
- Cost tracking by feature area

#### 7.2 Tracing Implementation
- Trace all LLM API calls with correlation IDs
- Log prompt/response pairs for debugging (with sanitization)
- Monitor for unexpected usage patterns
- Alert on cost or usage threshold breaches

### 8. Future LLM Integration Plans

#### 8.1 Advanced Capabilities
- Conversational AI for customer interaction
- Intelligent contact categorization and segmentation
- Predictive analytics powered by LLMs
- Natural language query for data exploration

#### 8.2 Integration Points
- Enhanced campaign personalization
- Automated event promotion creation
- Intelligent timeline analysis
- Predictive engagement scoring

### 9. Quality Assurance for LLM Outputs

#### 9.1 Content Validation
- Factual accuracy checking for generated content
- Brand consistency validation
- Compliance checking for marketing regulations
- Tone and style consistency verification

#### 9.2 Testing Approach
- Unit tests that mock LLM responses
- Integration tests with example prompts
- A/B testing for generated content effectiveness
- Human review workflows for sensitive content

### 10. Troubleshooting Common Issues

#### 10.1 Handling LLM Failures
- Graceful degradation when LLM services are unavailable
- Fallback to static templates or cached content
- Circuit breaker patterns for LLM service calls
- Error logging and alerting

#### 10.2 Managing Generation Quality
- Prompt refinement based on output quality
- Multiple generation attempts with selection criteria
- Human-in-the-loop validation for critical content
- Continuous improvement through feedback loops