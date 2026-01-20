# OpenAPI → LLM Tools Integration

This module provides ready-to-use LLM tool wrappers for the CRM API, enabling immediate integration with AI frameworks like LangChain, OpenAI function calling, and Claude tool use.

## Quick Start

### Prerequisites

```bash
pip install langchain langchain-openai langchain-anthropic requests
```

### Basic Usage with LangChain

```python
from crm_tools import CRMToolkit
from langchain_anthropic import ChatAnthropic

# Initialize
toolkit = CRMToolkit(base_url="http://localhost:8080")
llm = ChatAnthropic(model="claude-sonnet-4-20250514")

# Create agent
agent = toolkit.create_agent(llm)

# Run queries
response = agent.invoke("Find all leads tagged with 'techcrunch-2024'")
print(response)
```

### Direct Tool Usage

```python
from crm_tools import CRMToolkit

toolkit = CRMToolkit(base_url="http://localhost:8080")

# Search contacts
contacts = toolkit.search_contacts(status="lead", tags=["founder"])

# Log an interaction
toolkit.log_interaction(
    contact_id="abc123",
    type="meeting",
    content="Discussed product roadmap"
)
```

## Available Tools

| Tool | Description |
|------|-------------|
| `search_contacts` | Search contacts by name, status, tags, engagement |
| `get_contact` | Get detailed contact info with timeline |
| `create_contact` | Add a new contact |
| `update_contact` | Update contact details or status |
| `log_interaction` | Record meetings, calls, emails, notes |
| `list_campaigns` | Get active campaigns |
| `get_pipeline_summary` | Pipeline metrics and counts |

## Framework Support

### OpenAI Function Calling

```python
from crm_tools import get_openai_functions

functions = get_openai_functions()
# Use with OpenAI chat completions API
```

### Claude Tool Use

```python
from crm_tools import get_claude_tools

tools = get_claude_tools()
# Use with Anthropic messages API
```

### Raw OpenAPI Spec

The full OpenAPI spec is available at:
- Swagger UI: `http://localhost:8080/swagger-ui`
- JSON spec: `http://localhost:8080/api-docs/openapi.json`

## Example Workflows

See the `examples/` directory for:
- `crm_agent_demo.ipynb` - Interactive notebook with common workflows
- `langchain_agent.py` - Production-ready LangChain agent
- `openai_functions.py` - OpenAI function calling example
- `claude_tools.py` - Anthropic Claude tool use example
