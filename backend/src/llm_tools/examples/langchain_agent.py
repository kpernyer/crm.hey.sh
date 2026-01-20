#!/usr/bin/env python3
"""
LangChain Agent Example for CRM.HEY.SH

Demonstrates how to create an AI agent that can interact with your CRM
using natural language.

Prerequisites:
    pip install langchain langchain-anthropic requests

Usage:
    python langchain_agent.py
"""

import os
import sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from crm_tools import CRMToolkit


def main():
    # Check for API key
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Please set ANTHROPIC_API_KEY environment variable")
        print("export ANTHROPIC_API_KEY=your-api-key")
        sys.exit(1)

    # Initialize toolkit
    crm_url = os.environ.get("CRM_API_URL", "http://localhost:8080")
    toolkit = CRMToolkit(base_url=crm_url)

    # Initialize LLM
    try:
        from langchain_anthropic import ChatAnthropic
    except ImportError:
        print("Please install langchain-anthropic:")
        print("pip install langchain-anthropic")
        sys.exit(1)

    llm = ChatAnthropic(
        model="claude-sonnet-4-20250514",
        temperature=0,
    )

    # Create agent
    agent = toolkit.create_agent(llm, verbose=True)

    # Interactive loop
    print("\n" + "="*60)
    print("CRM AI Agent")
    print("="*60)
    print("\nI can help you manage your CRM. Try asking me to:")
    print("  - Find contacts (e.g., 'Find all leads from TechCrunch')")
    print("  - Get contact details (e.g., 'Tell me about John Smith')")
    print("  - Log interactions (e.g., 'Log that I had coffee with Sarah')")
    print("  - Check pipeline (e.g., 'How many leads do I have?')")
    print("\nType 'quit' to exit.\n")

    while True:
        try:
            user_input = input("You: ").strip()
            if not user_input:
                continue
            if user_input.lower() in ("quit", "exit", "q"):
                print("Goodbye!")
                break

            response = agent.invoke({"input": user_input})
            print(f"\nAgent: {response['output']}\n")

        except KeyboardInterrupt:
            print("\nGoodbye!")
            break
        except Exception as e:
            print(f"\nError: {e}\n")


if __name__ == "__main__":
    main()
