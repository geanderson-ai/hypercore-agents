# shim for developers: small Python API
# Em produção, hypercore_native estaria instalado no venv
try:
    from hypercore_native import register_tool, call_tool, llm_chat, web_search
except ImportError:
    # Fallback to help IDEs or when running without runtime (optional)
    def register_tool(name, fn): pass
    def call_tool(name, args): pass
    def llm_chat(prompt): return "No runtime"
    def web_search(query): return "No runtime"

def tool(fn):
    # decorator registers python function to runtime
    register_tool(fn.__name__, fn)
    return fn

class llm:
    @staticmethod
    def chat(prompt: str):
        return llm_chat(prompt)

class web:
    @staticmethod
    def search(q: str):
        return web_search(q)
