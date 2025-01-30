import random

from app.db.questions import (
    en_questions,
    pt_questions,
    it_questions,
)
from app.api.response import bad_request, server_error, success
from app.ml_models.gemini2 import generate_content
from app.utils.logger import get_logger
from app.utils.sde_prompt_template import get_sde_prompt, MAX_INPUT_TOKENS

logger = get_logger(__name__)


def core_random_question(lang, index):
    try:
        lang_prompts = {
            "portuguese": pt_questions,
            "italian": it_questions,
            "english": en_questions,
        }

        questions = lang_prompts.get(lang, en_questions)
        index = index if index else random.randint(0, len(questions) - 1)
        text_saved = questions[index]

        return success({"question": text_saved, "question_id": index})
    except Exception as e:
        error = str(e)
        logger.error("Error occurred while finding question:", error)
        return server_error({"error": error})


async def core_feedback(request, lang):
    try:
        question = request.question
        response = request.response

        prompt, sys_instructions, n_tokens = get_sde_prompt(question, response, lang)
        if n_tokens == 0:
            return bad_request({"error": f"Input exceeded {MAX_INPUT_TOKENS} tokens."})

        response = await generate_content(prompt, sys_instructions)
        return success({"message": response})

    except Exception as ex:
        error = str(ex)
        logger.error(f"Error occurred while requesting feedback: {error}")
        return server_error({"error": error})
