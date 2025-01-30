from pydantic import BaseModel


class ChatRequest(BaseModel):
    question: str
    response: str

    def __getattr__(self, name):
        attr = self.model_dump()
        if name in attr:
            return attr[name]
        raise AttributeError(
            f"'{self.__class__.__name__}' object has no attribute '{name}'"
        )
