FROM python:3.12-slim-bullseye

RUN apt-get update && apt-get install -y --no-install-recommends ffmpeg && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv

COPY . /app

WORKDIR /app
RUN uv sync --frozen --no-cache

EXPOSE 80

CMD ["/app/.venv/bin/uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "80"]
