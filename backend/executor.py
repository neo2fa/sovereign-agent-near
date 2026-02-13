from agent_engine import AgentEngine

def execute_action(code: str, action: str):
    engine = AgentEngine(code)
    hash_val = engine.compute_hash()
    print(f"Executing {action} with codehash: {hash_val}")
