"""
CRM LLM Tools - OpenAPI to LLM Integration

Quick Start:
    from llm_tools import CRMToolkit

    toolkit = CRMToolkit(base_url="http://localhost:8080")

    # LangChain
    agent = toolkit.create_agent(llm)

    # OpenAI
    functions = toolkit.get_openai_functions()

    # Claude
    tools = toolkit.get_claude_tools()
"""

from .crm_tools import (
    CRMToolkit,
    CRMClient,
    CRMConfig,
    get_openai_functions,
    get_claude_tools,
)

__all__ = [
    "CRMToolkit",
    "CRMClient",
    "CRMConfig",
    "get_openai_functions",
    "get_claude_tools",
]

__version__ = "0.1.0"
