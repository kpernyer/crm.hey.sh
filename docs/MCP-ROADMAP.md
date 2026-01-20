# MCP Integration Roadmap for CRM.HEY.SH

This document outlines the strategy for enabling LLM integration with the CRM, from quick wins to full MCP implementation.

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      LLM Integration Layers                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Phase 1: OpenAPI → LLM Tools (Quick Win)                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━                      │
│  • Expose existing REST API to LLM frameworks                   │
│  • Zero backend changes required                                │
│  • Works with LangChain, OpenAI plugins, etc.                  │
│                                                                 │
│  Phase 2: MCP Server (Full Integration)                        │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━                      │
│  • Native Model Context Protocol server                         │
│  • Optimized for LLM interactions                               │
│  • Works with Claude Desktop, Claude Code, etc.                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: OpenAPI → LLM Tools (Quick Win)

### What This Enables

Your existing REST API with Swagger/OpenAPI documentation can be directly consumed by LLM frameworks:

```python
# Example: LangChain consuming your OpenAPI spec
from langchain_community.tools.openapi import OpenAPIToolkit

toolkit = OpenAPIToolkit.from_openapi_url(
    "http://localhost:8080/api-docs/openapi.json"
)
agent = create_openapi_agent(llm, toolkit)
agent.run("Find all leads tagged with 'techcrunch-2024'")
```

### Implementation

Located at: `backend/src/llm_tools/`

This module provides:
1. **Enhanced OpenAPI descriptions** - Better tool descriptions for LLM consumption
2. **Python client generator** - Ready-to-use LangChain tools
3. **Example notebooks** - Demonstrating LLM workflows

### Usage

```bash
# Generate Python LLM tools from OpenAPI spec
just generate-llm-tools

# Run example notebook
cd backend/src/llm_tools/examples
jupyter notebook crm_agent_demo.ipynb
```

### Supported LLM Frameworks

| Framework | Support Level | Notes |
|-----------|--------------|-------|
| LangChain | Full | OpenAPI toolkit + custom tools |
| OpenAI Functions | Full | Via function calling JSON schema |
| Claude Tool Use | Full | Via tool definitions |
| AutoGPT | Partial | REST plugin format |

---

## Phase 2: MCP Server (Full Integration)

### What MCP Adds Over OpenAPI

| Feature | OpenAPI | MCP |
|---------|---------|-----|
| Tool discovery | Static spec | Dynamic |
| Streaming responses | No | Yes |
| Bidirectional communication | No | Yes |
| Context management | Manual | Built-in |
| Resource subscriptions | No | Yes |
| Optimized for LLMs | Adapted | Native |

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        CRM Backend                              │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                 Core Services Layer                       │  │
│  │   ContactService  CampaignService  AnalyticsService      │  │
│  └─────────────┬─────────────────────────────┬──────────────┘  │
│                │                             │                  │
│  ┌─────────────▼─────────────┐  ┌───────────▼──────────────┐  │
│  │      REST API (:8080)     │  │    MCP Server (:3001)    │  │
│  │                           │  │                          │  │
│  │  • Web/Mobile clients     │  │  • Claude Desktop        │  │
│  │  • Third-party apps       │  │  • Claude Code           │  │
│  │  • OpenAPI consumers      │  │  • Custom LLM agents     │  │
│  └───────────────────────────┘  └──────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### MCP Tools Specification

Located at: `backend/mcp-server/`

#### Contact Tools

```json
{
  "name": "search_contacts",
  "description": "Search CRM contacts. Use this to find people by name, company, status, tags, or engagement level. Returns contact summaries with IDs for further operations.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Free-text search across name, email, company"
      },
      "status": {
        "type": "string",
        "enum": ["lead", "customer", "partner", "investor"],
        "description": "Filter by pipeline status"
      },
      "tags": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Filter by tags (e.g., ['techcrunch-2024', 'founder'])"
      },
      "min_engagement": {
        "type": "number",
        "description": "Minimum engagement score (0-100)"
      },
      "limit": {
        "type": "integer",
        "default": 20,
        "description": "Maximum results to return"
      }
    }
  }
}
```

```json
{
  "name": "get_contact_details",
  "description": "Get full details and recent interaction history for a specific contact. Use after search_contacts to dive deeper.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "contact_id": {
        "type": "string",
        "description": "Contact ID from search results"
      },
      "include_timeline": {
        "type": "boolean",
        "default": true,
        "description": "Include recent interactions"
      },
      "timeline_limit": {
        "type": "integer",
        "default": 10,
        "description": "Number of timeline entries"
      }
    },
    "required": ["contact_id"]
  }
}
```

```json
{
  "name": "log_interaction",
  "description": "Record an interaction with a contact (meeting, call, email, note). Always log interactions to maintain relationship context.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "contact_id": {
        "type": "string",
        "description": "Contact ID"
      },
      "type": {
        "type": "string",
        "enum": ["email_sent", "call", "meeting", "note", "social_touch"],
        "description": "Type of interaction"
      },
      "content": {
        "type": "string",
        "description": "Summary or content of the interaction"
      },
      "metadata": {
        "type": "object",
        "description": "Additional structured data (e.g., meeting duration, topics discussed)"
      }
    },
    "required": ["contact_id", "type", "content"]
  }
}
```

```json
{
  "name": "update_contact",
  "description": "Update a contact's information or status. Use to move contacts through the pipeline or update their details.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "contact_id": { "type": "string" },
      "status": {
        "type": "string",
        "enum": ["lead", "customer", "partner", "investor", "other"]
      },
      "tags": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Replace existing tags"
      },
      "add_tags": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Add to existing tags"
      },
      "notes": { "type": "string" }
    },
    "required": ["contact_id"]
  }
}
```

```json
{
  "name": "create_contact",
  "description": "Add a new contact to the CRM. Use when you learn about a new person the user wants to track.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "first_name": { "type": "string" },
      "last_name": { "type": "string" },
      "email": { "type": "string" },
      "phone": { "type": "string" },
      "company": { "type": "string" },
      "linkedin_url": { "type": "string" },
      "status": {
        "type": "string",
        "enum": ["lead", "customer", "partner", "investor"],
        "default": "lead"
      },
      "tags": {
        "type": "array",
        "items": { "type": "string" }
      },
      "notes": { "type": "string" }
    },
    "required": ["first_name", "last_name"]
  }
}
```

#### Campaign Tools

```json
{
  "name": "suggest_campaign_contacts",
  "description": "Get AI-suggested contacts for a campaign based on objective and criteria. Use before creating outreach campaigns.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "objective": {
        "type": "string",
        "enum": ["awareness", "lead_gen", "event", "investor", "early_adopters"],
        "description": "Campaign goal"
      },
      "criteria": {
        "type": "string",
        "description": "Natural language description of ideal contacts"
      },
      "limit": {
        "type": "integer",
        "default": 50
      }
    },
    "required": ["objective"]
  }
}
```

```json
{
  "name": "draft_campaign_content",
  "description": "Generate draft content for a campaign (email, social post, landing page). Returns editable drafts.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "content_type": {
        "type": "string",
        "enum": ["email", "social_post", "landing_page", "event_invite"]
      },
      "context": {
        "type": "string",
        "description": "What the campaign is about"
      },
      "tone": {
        "type": "string",
        "enum": ["professional", "casual", "urgent", "friendly"],
        "default": "professional"
      },
      "target_audience": {
        "type": "string",
        "description": "Who this is for"
      }
    },
    "required": ["content_type", "context"]
  }
}
```

#### Analytics Tools

```json
{
  "name": "get_pipeline_summary",
  "description": "Get current pipeline status - how many contacts in each stage, conversion rates, engagement trends.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "time_range": {
        "type": "string",
        "enum": ["7d", "30d", "90d", "all"],
        "default": "30d"
      }
    }
  }
}
```

```json
{
  "name": "get_engagement_insights",
  "description": "Identify contacts needing attention - stale leads, highly engaged prospects, recent converts.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "insight_type": {
        "type": "string",
        "enum": ["stale_leads", "hot_prospects", "recent_activity", "needs_followup"]
      },
      "limit": {
        "type": "integer",
        "default": 10
      }
    },
    "required": ["insight_type"]
  }
}
```

---

## Typical LLM Workflows

### 1. Post-Event Follow-up

```
User: "I just got back from TechCrunch Disrupt. Help me follow up with everyone I met."

LLM Actions:
1. search_contacts(tags=["techcrunch-2024"]) → Find event contacts
2. get_contact_details(id) → Get context on each
3. [Generate personalized follow-up emails]
4. log_interaction(type="email_sent") → Record outreach
```

### 2. Investor Pipeline Review

```
User: "How's my investor outreach going? Who should I focus on?"

LLM Actions:
1. search_contacts(status="investor") → Get all investors
2. get_engagement_insights(type="hot_prospects") → Find engaged ones
3. get_engagement_insights(type="needs_followup") → Find stale ones
4. [Provide summary and recommendations]
```

### 3. Quick Contact Logging

```
User: "Just had coffee with Sarah Chen from Acme Corp. Discussed their Series A."

LLM Actions:
1. search_contacts(query="Sarah Chen Acme") → Find contact
2. log_interaction(
     contact_id=...,
     type="meeting",
     content="Coffee meeting - discussed Series A fundraise",
     metadata={location: "coffee", topics: ["series_a"]}
   )
3. [Confirm logged]
```

### 4. Campaign Creation

```
User: "Create a campaign for early-stage founders who might be beta users"

LLM Actions:
1. suggest_campaign_contacts(
     objective="early_adopters",
     criteria="founders at seed/pre-seed startups"
   )
2. draft_campaign_content(
     content_type="email",
     context="beta user recruitment",
     target_audience="early-stage founders"
   )
3. [Present contacts + draft for approval]
```

---

## MCP Resources (Read-Only Data)

In addition to tools, MCP supports "resources" for read-only data access:

```json
{
  "resources": [
    {
      "uri": "crm://contacts/recent",
      "name": "Recent Contacts",
      "description": "Contacts added or updated in the last 7 days"
    },
    {
      "uri": "crm://pipeline/summary",
      "name": "Pipeline Summary",
      "description": "Current contact counts by status"
    },
    {
      "uri": "crm://contacts/{id}",
      "name": "Contact Details",
      "description": "Full details for a specific contact"
    }
  ]
}
```

---

## Implementation Priority

### Phase 1: Quick Win (Now)
1. ✅ OpenAPI spec already exists at `/swagger-ui`
2. Add enhanced descriptions for LLM consumption
3. Create Python examples with LangChain
4. Document usage patterns

### Phase 2: MCP Foundation
1. Create `mcp-server` crate
2. Implement core tools: `search_contacts`, `get_contact_details`, `log_interaction`
3. Add stdio transport for Claude Desktop
4. Test with Claude Code

### Phase 3: Full MCP
1. Add remaining tools
2. Implement resources
3. Add HTTP+SSE transport for web clients
4. Add authentication/authorization

---

## Configuration

### Claude Desktop Integration

Add to `~/.config/claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "crm": {
      "command": "/path/to/crm-mcp-server",
      "args": ["--db-url", "ws://localhost:8000"],
      "env": {
        "CRM_API_KEY": "your-api-key"
      }
    }
  }
}
```

### Environment Variables

```bash
# MCP Server Configuration
MCP_TRANSPORT=stdio          # stdio | http | websocket
MCP_HTTP_PORT=3001           # Port for HTTP transport
MCP_AUTH_REQUIRED=true       # Require API key authentication

# Database (reuses main backend config)
CRM__DATABASE__URL=ws://localhost:8000
CRM__DATABASE__NAMESPACE=crm
CRM__DATABASE__DATABASE=main
```

---

## Security Considerations

1. **Authentication**: MCP server should validate API keys
2. **Authorization**: Respect user permissions from main app
3. **Rate Limiting**: Prevent LLM loops from overwhelming the API
4. **Audit Logging**: Log all MCP tool calls for compliance
5. **Data Filtering**: Ensure LLM can only access authorized data

---

## Comparison: When to Use What

| Use Case | Approach |
|----------|----------|
| Claude Desktop / Claude Code | MCP Server |
| Custom Python LLM app | OpenAPI + LangChain |
| OpenAI GPT integration | OpenAPI → Function Calling |
| Web-based AI assistant | MCP over HTTP+SSE |
| Quick prototyping | OpenAPI (zero setup) |
| Production LLM workflows | MCP (full features) |
