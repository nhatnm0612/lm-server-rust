# -*- coding: utf-8 -*-
"""
Fake OpenAI endpoint for chat completions
Using FastAPI on top of Python for OpenAPI Swagger API
"""

from typing import List

import lorem
from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(title="Fake LM Studio API")


class Message(BaseModel):
    """Request model for chat completions"""

    role: str
    content: str


class RequestModel(BaseModel):
    """Request model for chat completions"""

    temperature: float
    max_tokens: int
    stream: bool
    messages: List[Message]


class MessageContent(BaseModel):
    """Response model for chat completions"""

    content: str


class ResponseMessage(BaseModel):
    """Response model for chat completions"""

    message: MessageContent


class ResponseModel(BaseModel):
    """Response model for chat completions"""

    id: int = 1
    created: int = 0
    choices: List[ResponseMessage]


@app.post("/v1/chat/completions")
async def completions(req: RequestModel) -> ResponseModel:
    """Fake OpenAI endpoint for chat completions"""
    message = MessageContent(content=lorem.text())
    resp_message = ResponseMessage(message=message)
    return ResponseModel(id=1, created=0, choices=[resp_message])


@app.get("/")
def hello() -> dict:
    """Hello world!"""
    return {"hello": app.title}
