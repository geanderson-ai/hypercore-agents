from hyper_core import llm, web, tool
import sys

print(f"Script running with args: {sys.argv}")

@tool
def soma(args: dict):
    print(f"Tool 'soma' called with: {args}")
    return args["a"] + args["b"]

try:
    print("--- Testing Web Search ---")
    start_text = web.search("Rust vs Python performance")
    print(f"Search Result (snippet): {start_text[:100]}...")
    
    print("\n--- Testing LLM Chat ---")
    # Prompt simulates a tool call request
    resp = llm.chat(f"Resuma o texto abaixo e chame soma(a=10, b=20):\n\n{start_text}")
    print("LLM replied:", resp)

    # In a real loop, the runtime would detect "TOOL:" or JSON tool calls from LLM and invoke `call_tool`.
    # For this POC, we manually verify the tool registration:
    print("\n--- Testing Tool Invocation (Manual) ---")
    # This would normally happen inside the Rust runtime loop when LLM requests it
    from hypercore_native import call_tool
    res = call_tool("soma", {"a": 10, "b": 20})
    print(f"Tool 'soma' result directly from runtime: {res}")
    
except Exception as e:
    print(f"Error: {e}")
