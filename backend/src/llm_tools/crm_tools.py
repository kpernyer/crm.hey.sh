"""
CRM Tools for LLM Integration

Provides ready-to-use tool wrappers for integrating CRM.HEY.SH with LLM frameworks
like LangChain, OpenAI function calling, and Claude tool use.

Usage:
    from crm_tools import CRMToolkit

    toolkit = CRMToolkit(base_url="http://localhost:8080")
    agent = toolkit.create_agent(llm)
    response = agent.invoke("Find all leads from TechCrunch")
"""

import json
from typing import Any, Dict, List, Optional, Type
from dataclasses import dataclass
import requests

# LangChain imports (optional, graceful fallback)
try:
    from langchain_core.tools import BaseTool, ToolException
    from langchain_core.callbacks import CallbackManagerForToolRun
    from langchain.agents import AgentExecutor, create_tool_calling_agent
    from langchain_core.prompts import ChatPromptTemplate
    from pydantic import BaseModel, Field
    LANGCHAIN_AVAILABLE = True
except ImportError:
    LANGCHAIN_AVAILABLE = False
    BaseTool = object
    BaseModel = object
    Field = lambda *args, **kwargs: None


@dataclass
class CRMConfig:
    """Configuration for CRM API connection."""
    base_url: str = "http://localhost:8080"
    api_key: Optional[str] = None
    timeout: int = 30


class CRMClient:
    """Low-level HTTP client for CRM API."""

    def __init__(self, config: CRMConfig):
        self.config = config
        self.session = requests.Session()
        if config.api_key:
            self.session.headers["Authorization"] = f"Bearer {config.api_key}"
        self.session.headers["Content-Type"] = "application/json"

    def _url(self, path: str) -> str:
        return f"{self.config.base_url}{path}"

    def get(self, path: str, params: Optional[Dict] = None) -> Dict:
        resp = self.session.get(self._url(path), params=params, timeout=self.config.timeout)
        resp.raise_for_status()
        return resp.json()

    def post(self, path: str, data: Dict) -> Dict:
        resp = self.session.post(self._url(path), json=data, timeout=self.config.timeout)
        resp.raise_for_status()
        return resp.json()

    def patch(self, path: str, data: Dict) -> Dict:
        resp = self.session.patch(self._url(path), json=data, timeout=self.config.timeout)
        resp.raise_for_status()
        return resp.json()

    def delete(self, path: str) -> Dict:
        resp = self.session.delete(self._url(path), timeout=self.config.timeout)
        resp.raise_for_status()
        return resp.json()


# =============================================================================
# Tool Input Schemas (Pydantic models for LangChain)
# =============================================================================

if LANGCHAIN_AVAILABLE:
    class SearchContactsInput(BaseModel):
        """Input for searching contacts."""
        query: Optional[str] = Field(None, description="Free-text search across name, email, company")
        status: Optional[str] = Field(None, description="Filter by status: lead, customer, partner, investor")
        tags: Optional[List[str]] = Field(None, description="Filter by tags")
        min_engagement: Optional[float] = Field(None, description="Minimum engagement score (0-100)")
        limit: int = Field(20, description="Maximum results to return")

    class GetContactInput(BaseModel):
        """Input for getting contact details."""
        contact_id: str = Field(..., description="The contact ID")
        include_timeline: bool = Field(True, description="Include recent interactions")

    class CreateContactInput(BaseModel):
        """Input for creating a contact."""
        first_name: str = Field(..., description="Contact's first name")
        last_name: str = Field(..., description="Contact's last name")
        email: Optional[str] = Field(None, description="Email address")
        phone: Optional[str] = Field(None, description="Phone number")
        company: Optional[str] = Field(None, description="Company name")
        status: str = Field("lead", description="Initial status: lead, customer, partner, investor")
        tags: Optional[List[str]] = Field(None, description="Tags to categorize")
        notes: Optional[str] = Field(None, description="Initial notes")

    class UpdateContactInput(BaseModel):
        """Input for updating a contact."""
        contact_id: str = Field(..., description="Contact ID to update")
        status: Optional[str] = Field(None, description="New status")
        tags: Optional[List[str]] = Field(None, description="Replace tags")
        add_tags: Optional[List[str]] = Field(None, description="Tags to add")

    class LogInteractionInput(BaseModel):
        """Input for logging an interaction."""
        contact_id: str = Field(..., description="Contact ID")
        type: str = Field(..., description="Type: email_sent, call, meeting, note, social_touch")
        content: str = Field(..., description="Summary of the interaction")
        metadata: Optional[Dict] = Field(None, description="Additional data (duration, topics, etc.)")

    class GetPipelineSummaryInput(BaseModel):
        """Input for pipeline summary."""
        time_range: str = Field("30d", description="Time range: 7d, 30d, 90d, all")


# =============================================================================
# LangChain Tools
# =============================================================================

if LANGCHAIN_AVAILABLE:
    class SearchContactsTool(BaseTool):
        """Search CRM contacts."""
        name: str = "search_contacts"
        description: str = """Search CRM contacts by name, company, status, tags, or engagement level.
        Use this to find people matching specific criteria. Returns contact summaries with IDs."""
        args_schema: Type[BaseModel] = SearchContactsInput
        client: Any = None

        def _run(
            self,
            query: Optional[str] = None,
            status: Optional[str] = None,
            tags: Optional[List[str]] = None,
            min_engagement: Optional[float] = None,
            limit: int = 20,
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            params = {"limit": limit}
            if query:
                params["query"] = query
            if status:
                params["status"] = status
            if tags:
                params["tags"] = ",".join(tags)
            if min_engagement:
                params["min_engagement"] = min_engagement

            try:
                result = self.client.get("/api/contacts", params)
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to search contacts: {e}")

    class GetContactTool(BaseTool):
        """Get contact details with timeline."""
        name: str = "get_contact"
        description: str = """Get full details and recent interaction history for a specific contact.
        Use after search_contacts to dive deeper into a contact's profile."""
        args_schema: Type[BaseModel] = GetContactInput
        client: Any = None

        def _run(
            self,
            contact_id: str,
            include_timeline: bool = True,
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            try:
                result = self.client.get(f"/api/contacts/{contact_id}")
                if include_timeline:
                    timeline = self.client.get(f"/api/contacts/{contact_id}/timeline")
                    result["timeline"] = timeline
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to get contact: {e}")

    class CreateContactTool(BaseTool):
        """Create a new contact."""
        name: str = "create_contact"
        description: str = """Add a new contact to the CRM. Use when you learn about a new person
        the user wants to track. Requires at least first and last name."""
        args_schema: Type[BaseModel] = CreateContactInput
        client: Any = None

        def _run(
            self,
            first_name: str,
            last_name: str,
            email: Optional[str] = None,
            phone: Optional[str] = None,
            company: Optional[str] = None,
            status: str = "lead",
            tags: Optional[List[str]] = None,
            notes: Optional[str] = None,
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            data = {
                "first_name": first_name,
                "last_name": last_name,
                "status": status,
            }
            if email:
                data["email"] = email
            if phone:
                data["phone"] = phone
            if company:
                data["company"] = company
            if tags:
                data["tags"] = tags
            if notes:
                data["notes"] = notes

            try:
                result = self.client.post("/api/contacts", data)
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to create contact: {e}")

    class UpdateContactTool(BaseTool):
        """Update a contact."""
        name: str = "update_contact"
        description: str = """Update a contact's information or status. Use to move contacts
        through the pipeline or update their details."""
        args_schema: Type[BaseModel] = UpdateContactInput
        client: Any = None

        def _run(
            self,
            contact_id: str,
            status: Optional[str] = None,
            tags: Optional[List[str]] = None,
            add_tags: Optional[List[str]] = None,
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            data = {}
            if status:
                data["status"] = status
            if tags:
                data["tags"] = tags
            # Note: add_tags would need backend support

            try:
                result = self.client.patch(f"/api/contacts/{contact_id}", data)
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to update contact: {e}")

    class LogInteractionTool(BaseTool):
        """Log an interaction with a contact."""
        name: str = "log_interaction"
        description: str = """Record an interaction with a contact (meeting, call, email, note).
        Always log interactions to maintain relationship context and history."""
        args_schema: Type[BaseModel] = LogInteractionInput
        client: Any = None

        def _run(
            self,
            contact_id: str,
            type: str,
            content: str,
            metadata: Optional[Dict] = None,
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            data = {
                "contact": contact_id,
                "type": type,
                "content": content,
            }
            if metadata:
                data["metadata"] = metadata

            try:
                result = self.client.post("/api/timeline", data)
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to log interaction: {e}")

    class GetPipelineSummaryTool(BaseTool):
        """Get pipeline summary."""
        name: str = "get_pipeline_summary"
        description: str = """Get current pipeline status - how many contacts in each stage,
        conversion rates, and engagement trends."""
        args_schema: Type[BaseModel] = GetPipelineSummaryInput
        client: Any = None

        def _run(
            self,
            time_range: str = "30d",
            run_manager: Optional[CallbackManagerForToolRun] = None,
        ) -> str:
            try:
                result = self.client.get("/api/analytics/contacts", {"time_range": time_range})
                return json.dumps(result, indent=2)
            except Exception as e:
                raise ToolException(f"Failed to get pipeline summary: {e}")


# =============================================================================
# Main Toolkit Class
# =============================================================================

class CRMToolkit:
    """
    CRM Tools for LLM Integration.

    Provides ready-to-use tools for LangChain, OpenAI, and Claude integrations.

    Usage:
        toolkit = CRMToolkit(base_url="http://localhost:8080")

        # LangChain
        agent = toolkit.create_agent(llm)

        # OpenAI
        functions = toolkit.get_openai_functions()

        # Claude
        tools = toolkit.get_claude_tools()

        # Direct usage
        contacts = toolkit.search_contacts(status="lead")
    """

    def __init__(
        self,
        base_url: str = "http://localhost:8080",
        api_key: Optional[str] = None,
    ):
        self.config = CRMConfig(base_url=base_url, api_key=api_key)
        self.client = CRMClient(self.config)

    # -------------------------------------------------------------------------
    # Direct API Methods
    # -------------------------------------------------------------------------

    def search_contacts(
        self,
        query: Optional[str] = None,
        status: Optional[str] = None,
        tags: Optional[List[str]] = None,
        min_engagement: Optional[float] = None,
        limit: int = 20,
    ) -> List[Dict]:
        """Search contacts directly."""
        params = {"limit": limit}
        if query:
            params["query"] = query
        if status:
            params["status"] = status
        if tags:
            params["tags"] = ",".join(tags)
        if min_engagement:
            params["min_engagement"] = min_engagement
        return self.client.get("/api/contacts", params)

    def get_contact(self, contact_id: str, include_timeline: bool = True) -> Dict:
        """Get contact details."""
        result = self.client.get(f"/api/contacts/{contact_id}")
        if include_timeline:
            result["timeline"] = self.client.get(f"/api/contacts/{contact_id}/timeline")
        return result

    def create_contact(self, **kwargs) -> Dict:
        """Create a new contact."""
        return self.client.post("/api/contacts", kwargs)

    def update_contact(self, contact_id: str, **kwargs) -> Dict:
        """Update a contact."""
        return self.client.patch(f"/api/contacts/{contact_id}", kwargs)

    def log_interaction(
        self,
        contact_id: str,
        type: str,
        content: str,
        metadata: Optional[Dict] = None,
    ) -> Dict:
        """Log an interaction."""
        data = {"contact": contact_id, "type": type, "content": content}
        if metadata:
            data["metadata"] = metadata
        return self.client.post("/api/timeline", data)

    def get_pipeline_summary(self, time_range: str = "30d") -> Dict:
        """Get pipeline summary."""
        return self.client.get("/api/analytics/contacts", {"time_range": time_range})

    # -------------------------------------------------------------------------
    # LangChain Integration
    # -------------------------------------------------------------------------

    def get_langchain_tools(self) -> List:
        """Get LangChain tool instances."""
        if not LANGCHAIN_AVAILABLE:
            raise ImportError("LangChain not installed. Run: pip install langchain")

        tools = [
            SearchContactsTool(client=self.client),
            GetContactTool(client=self.client),
            CreateContactTool(client=self.client),
            UpdateContactTool(client=self.client),
            LogInteractionTool(client=self.client),
            GetPipelineSummaryTool(client=self.client),
        ]
        return tools

    def create_agent(self, llm, verbose: bool = False):
        """Create a LangChain agent with CRM tools."""
        if not LANGCHAIN_AVAILABLE:
            raise ImportError("LangChain not installed. Run: pip install langchain")

        tools = self.get_langchain_tools()

        prompt = ChatPromptTemplate.from_messages([
            ("system", """You are a helpful CRM assistant. You help users manage their contacts,
            log interactions, and understand their pipeline. Always be concise and actionable.
            When searching for contacts, start with broad searches and narrow down.
            Always log important interactions to maintain relationship history."""),
            ("human", "{input}"),
            ("placeholder", "{agent_scratchpad}"),
        ])

        agent = create_tool_calling_agent(llm, tools, prompt)
        return AgentExecutor(agent=agent, tools=tools, verbose=verbose)

    # -------------------------------------------------------------------------
    # OpenAI Function Calling
    # -------------------------------------------------------------------------

    def get_openai_functions(self) -> List[Dict]:
        """Get function definitions for OpenAI function calling."""
        return [
            {
                "name": "search_contacts",
                "description": "Search CRM contacts by name, status, tags, or engagement level",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Free-text search"},
                        "status": {"type": "string", "enum": ["lead", "customer", "partner", "investor"]},
                        "tags": {"type": "array", "items": {"type": "string"}},
                        "min_engagement": {"type": "number"},
                        "limit": {"type": "integer", "default": 20},
                    },
                },
            },
            {
                "name": "get_contact",
                "description": "Get full details for a specific contact",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": {"type": "string", "description": "Contact ID"},
                        "include_timeline": {"type": "boolean", "default": True},
                    },
                    "required": ["contact_id"],
                },
            },
            {
                "name": "create_contact",
                "description": "Add a new contact to the CRM",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "first_name": {"type": "string"},
                        "last_name": {"type": "string"},
                        "email": {"type": "string"},
                        "phone": {"type": "string"},
                        "company": {"type": "string"},
                        "status": {"type": "string", "enum": ["lead", "customer", "partner", "investor"]},
                        "tags": {"type": "array", "items": {"type": "string"}},
                        "notes": {"type": "string"},
                    },
                    "required": ["first_name", "last_name"],
                },
            },
            {
                "name": "log_interaction",
                "description": "Record an interaction with a contact",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": {"type": "string"},
                        "type": {"type": "string", "enum": ["email_sent", "call", "meeting", "note", "social_touch"]},
                        "content": {"type": "string"},
                        "metadata": {"type": "object"},
                    },
                    "required": ["contact_id", "type", "content"],
                },
            },
            {
                "name": "get_pipeline_summary",
                "description": "Get pipeline metrics and contact counts by status",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "time_range": {"type": "string", "enum": ["7d", "30d", "90d", "all"]},
                    },
                },
            },
        ]

    def handle_openai_function_call(self, function_name: str, arguments: Dict) -> str:
        """Execute an OpenAI function call and return the result."""
        handlers = {
            "search_contacts": lambda args: self.search_contacts(**args),
            "get_contact": lambda args: self.get_contact(**args),
            "create_contact": lambda args: self.create_contact(**args),
            "log_interaction": lambda args: self.log_interaction(**args),
            "get_pipeline_summary": lambda args: self.get_pipeline_summary(**args),
        }

        if function_name not in handlers:
            return json.dumps({"error": f"Unknown function: {function_name}"})

        try:
            result = handlers[function_name](arguments)
            return json.dumps(result, indent=2)
        except Exception as e:
            return json.dumps({"error": str(e)})

    # -------------------------------------------------------------------------
    # Claude Tool Use
    # -------------------------------------------------------------------------

    def get_claude_tools(self) -> List[Dict]:
        """Get tool definitions for Claude tool use (Anthropic API)."""
        return [
            {
                "name": "search_contacts",
                "description": "Search CRM contacts by name, company, status, tags, or engagement level. Returns contact summaries with IDs for further operations.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Free-text search across name, email, company"},
                        "status": {"type": "string", "enum": ["lead", "customer", "partner", "investor"], "description": "Filter by pipeline status"},
                        "tags": {"type": "array", "items": {"type": "string"}, "description": "Filter by tags"},
                        "min_engagement": {"type": "number", "description": "Minimum engagement score (0-100)"},
                        "limit": {"type": "integer", "description": "Maximum results to return"},
                    },
                },
            },
            {
                "name": "get_contact",
                "description": "Get full details and recent interaction history for a specific contact. Use after search_contacts to dive deeper.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "contact_id": {"type": "string", "description": "Contact ID from search results"},
                        "include_timeline": {"type": "boolean", "description": "Include recent interactions"},
                    },
                    "required": ["contact_id"],
                },
            },
            {
                "name": "create_contact",
                "description": "Add a new contact to the CRM. Use when you learn about a new person the user wants to track.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "first_name": {"type": "string"},
                        "last_name": {"type": "string"},
                        "email": {"type": "string"},
                        "phone": {"type": "string"},
                        "company": {"type": "string"},
                        "status": {"type": "string", "enum": ["lead", "customer", "partner", "investor"]},
                        "tags": {"type": "array", "items": {"type": "string"}},
                        "notes": {"type": "string"},
                    },
                    "required": ["first_name", "last_name"],
                },
            },
            {
                "name": "log_interaction",
                "description": "Record an interaction with a contact (meeting, call, email, note). Always log interactions to maintain relationship context.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "contact_id": {"type": "string"},
                        "type": {"type": "string", "enum": ["email_sent", "call", "meeting", "note", "social_touch"]},
                        "content": {"type": "string", "description": "Summary of the interaction"},
                        "metadata": {"type": "object", "description": "Additional data (duration, topics, etc.)"},
                    },
                    "required": ["contact_id", "type", "content"],
                },
            },
            {
                "name": "get_pipeline_summary",
                "description": "Get current pipeline status - how many contacts in each stage, conversion rates, engagement trends.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "time_range": {"type": "string", "enum": ["7d", "30d", "90d", "all"]},
                    },
                },
            },
        ]

    def handle_claude_tool_use(self, tool_name: str, tool_input: Dict) -> str:
        """Execute a Claude tool use and return the result."""
        # Same as OpenAI function call handler
        return self.handle_openai_function_call(tool_name, tool_input)


# =============================================================================
# Convenience Functions
# =============================================================================

def get_openai_functions(base_url: str = "http://localhost:8080") -> List[Dict]:
    """Get OpenAI function definitions."""
    return CRMToolkit(base_url).get_openai_functions()


def get_claude_tools(base_url: str = "http://localhost:8080") -> List[Dict]:
    """Get Claude tool definitions."""
    return CRMToolkit(base_url).get_claude_tools()


if __name__ == "__main__":
    # Quick test
    toolkit = CRMToolkit()
    print("OpenAI Functions:")
    print(json.dumps(toolkit.get_openai_functions(), indent=2))
    print("\nClaude Tools:")
    print(json.dumps(toolkit.get_claude_tools(), indent=2))
