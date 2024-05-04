"""Testing code"""

from fastapi.testclient import TestClient
from dev.main import app  # pylint: disable=import-error


client = TestClient(app=app)


def test_get_hello():
    """Testing code"""
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"hello": "Fake LM Studio API"}


def test_post_completions():
    """Testing code"""
    response = client.post(
        "/v1/chat/completions",
        headers={"content-type": "application/json"},
        json={
            "temperature": 0.1,
            "max_tokens": 12,
            "stream": False,
            "messages": [
                {
                    "role": "user",
                    "content": "what is the last letter of the alphabet?",
                }
            ]
        }
    )
    assert response.status_code == 200
    response_json = response.json()
    assert "id" in response_json.keys()
    assert "created" in response_json.keys()
    assert "choices" in response_json.keys()
