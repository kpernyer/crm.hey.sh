#!/usr/bin/env python3
"""
OpenAI Function Calling Example for CRM.HEY.SH

Demonstrates how to use OpenAI's function calling feature with the CRM API.

Prerequisites:
    pip install openai requests

Usage:
    export OPENAI_API_KEY=your-api-key
    python openai_functions.py
"""

import os
import sys
import json

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from crm_tools import CRMToolkit

try:
    from openai import OpenAI
except ImportError:
    print("Please install openai:")
    print("pip install openai")
    sys.exit(1)


def run_conversation(client: OpenAI, toolkit: CRMToolkit, user_message: str):
    """Run a conversation with function calling."""

    functions = toolkit.get_openai_functions()
    messages = [
        {
            "role": "system",
            "content": "You are a helpful CRM assistant. Help users manage contacts and track relationships. Be concise."
        },
        {"role": "user", "content": user_message}
    ]

    print(f"\n{'='*60}")
    print(f"User: {user_message}")
    print("="*60)

    # Initial request
    response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=messages,
        functions=functions,
        function_call="auto",
    )

    message = response.choices[0].message

    # Handle function calls
    while message.function_call:
        function_name = message.function_call.name
        function_args = json.loads(message.function_call.arguments)

        print(f"\n[Calling function: {function_name}]")
        print(f"  Arguments: {json.dumps(function_args, indent=2)}")

        # Execute the function
        result = toolkit.handle_openai_function_call(function_name, function_args)
        print(f"  Result: {result[:200]}..." if len(result) > 200 else f"  Result: {result}")

        # Add function result to messages
        messages.append(message)
        messages.append({
            "role": "function",
            "name": function_name,
            "content": result,
        })

        # Continue conversation
        response = client.chat.completions.create(
            model="gpt-4-turbo-preview",
            messages=messages,
            functions=functions,
            function_call="auto",
        )

        message = response.choices[0].message

    # Final response
    print(f"\nAssistant: {message.content}")
    return message.content


def main():
    # Check for API key
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        print("Please set OPENAI_API_KEY environment variable")
        print("export OPENAI_API_KEY=your-api-key")
        sys.exit(1)

    # Initialize
    client = OpenAI()
    crm_url = os.environ.get("CRM_API_URL", "http://localhost:8080")
    toolkit = CRMToolkit(base_url=crm_url)

    # Example queries
    example_queries = [
        "Find all my investor contacts",
        "What's my current pipeline breakdown?",
        "Create a new lead: John Doe from Acme Corp, john@acme.com",
    ]

    print("\n" + "="*60)
    print("OpenAI Function Calling Demo - CRM.HEY.SH")
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
        except Exception as e:
            print(f"\nError: {e}")


if __name__ == "__main__":
    main()
