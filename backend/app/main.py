from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.api.routes import api_router
from app.api.response import OK
from app.utils.logger import get_logger

logger = get_logger(__name__)

app = FastAPI(
    title="SDE Behavioral Interview App",
)


app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
def health():
    return OK()


app.include_router(
    api_router,
)
