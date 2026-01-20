#!/usr/bin/env python3
"""
Claude Tool Use Example for CRM.HEY.SH

Demonstrates how to use Claude's native tool use feature with the CRM API.
This uses the Anthropic Python SDK directly (not LangChain).

Prerequisites:
    pip install anthropic requests

Usage:
    export ANTHROPIC_API_KEY=your-api-key
    python claude_tools.py
"""

import os
import sys
import json

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from crm_tools import CRMToolkit

try:
    import anthropic
except ImportError:
    print("Please install anthropic:")
    print("pip install anthropic")
    sys.exit(1)


def run_conversation(client: anthropic.Anthropic, toolkit: CRMToolkit, user_message: str):
    """Run a conversation with tool use."""

    tools = toolkit.get_claude_tools()
    messages = [{"role": "user", "content": user_message}]

    print(f"\n{'='*60}")
    print(f"User: {user_message}")
    print("="*60)

    # Initial request
    response = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=4096,
        tools=tools,
        messages=messages,
        system="You are a helpful CRM assistant. Help users manage contacts and track relationships."
    )

    # Handle tool use loop
    while response.stop_reason == "tool_use":
        # Extract tool use blocks
        tool_uses = [block for block in response.content if block.type == "tool_use"]

        # Process each tool use
        tool_results = []
        for tool_use in tool_uses:
            print(f"\n[Calling tool: {tool_use.name}]")
            print(f"  Input: {json.dumps(tool_use.input, indent=2)}")

            # Execute the tool
            result = toolkit.handle_claude_tool_use(tool_use.name, tool_use.input)
            print(f"  Result: {result[:200]}..." if len(result) > 200 else f"  Result: {result}")

            tool_results.append({
                "type": "tool_result",
                "tool_use_id": tool_use.id,
                "content": result,
            })

        # Continue conversation with tool results
        messages = [
            {"role": "user", "content": user_message},
            {"role": "assistant", "content": response.content},
            {"role": "user", "content": tool_results},
        ]

        response = client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=4096,
            tools=tools,
            messages=messages,
            system="You are a helpful CRM assistant. Help users manage contacts and track relationships."
        )

    # Extract final text response
    final_text = ""
    for block in response.content:
        if hasattr(block, "text"):
            final_text += block.text

    print(f"\nAssistant: {final_text}")
    return final_text


def main():
    # Check for API key
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Please set ANTHROPIC_API_KEY environment variable")
        print("export ANTHROPIC_API_KEY=your-api-key")
        sys.exit(1)

    # Initialize
    client = anthropic.Anthropic()
    crm_url = os.environ.get("CRM_API_URL", "http://localhost:8080")
    toolkit = CRMToolkit(base_url=crm_url)

    # Example queries
    example_queries = [
        "Find all leads tagged with 'founder'",
        "How many contacts do I have in each pipeline stage?",
        "Show me the details for contact contact:abc123",
    ]

    print("\n" + "="*60)
    print("Claude Tool Use Demo - CRM.HEY.SH")
    print("="*60)
    print("\nExample queries you can try:")
    for i, q in enumerate(example_queries, 1):
        print(f"  {i}. {q}")
    print("\nType 'quit' to exit.\n")

    # Interactive loop
    while True:
        try:
            user_input = input("\nYou: ").strip()
            if not user_input:
                continue
            if user_input.lower() in ("quit", "exit", "q"):
                print("Goodbye!")
                break

            run_conversation(client, toolkit, user_input)

        except KeyboardInterrupt:
            print("\nGoodbye!")
            break
        except anthropic.APIError as e:
            print(f"\nAPI Error: {e}")
        except Exception as e:
            print(f"\nError: {e}")


if __name__ == "__main__":
    main()
