# Run it
Make sure you have *uv* installed. More info here: https://github.com/astral-sh/uv

- Install dependencies: `uv sync` 

- Start the server: uvicorn app.main:app --reload --env-file=.env

# Environment variables

GEMINI_API_KEY=your_key

Get your key here: https://console.cloud.google.com/apis/credentials
