from fastapi import APIRouter, File, UploadFile
from fastapi.params import Query

from app.ml_models.whisper import WhisperModelSingleton
from app.models.chat import ChatRequest
from app.core.core import core_feedback, core_random_question
from app.utils.logger import get_logger


logger = get_logger(__name__)

api_router = APIRouter()

whisper_singleton = WhisperModelSingleton()


@api_router.get("/get-random-question")
async def get_random_question(
    lang: str = Query(default=None), index: int = Query(default=None)
):
    return core_random_question(lang, index)


@api_router.post("/chat")
async def feedback(request: ChatRequest, lang: str = Query(default=None)):
    return await core_feedback(request, lang)


@api_router.post("/transcribe")
async def transcribe_audio(
    file: UploadFile = File(...),
    lang: str = Query(None),
    save: str = Query(None),
):
    return await whisper_singleton.output(file, save, lang)
