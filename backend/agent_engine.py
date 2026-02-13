import hashlib

class AgentEngine:
    def __init__(self, code: str):
        self.code = code

    def compute_hash(self):
        return hashlib.sha256(self.code.encode()).hexdigest()
