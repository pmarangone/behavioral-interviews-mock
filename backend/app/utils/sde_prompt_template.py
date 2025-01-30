from app.db.prompt_template import en_prompt, pt_prompt, it_prompt
from app.utils.logger import get_logger

logger = get_logger(__name__)

# avoid extrapolating google AI api token limit
MAX_INPUT_TOKENS = 80_000


def get_sde_prompt(question, response, lang) -> str:
    input_len = len(response)
    if input_len > MAX_INPUT_TOKENS:
        logger.error(f"Input length exceeded {MAX_INPUT_TOKENS} tokens.")
        return "", 0

    lang_prompts = {
        "portuguese": pt_prompt,
        "italian": it_prompt,
        "english": en_prompt,
    }

    system_instructions = lang_prompts.get(lang, en_prompt)

    prompt = f""" 
Question: {question}\n
Response: {response}
"""

    logger.debug("input len:", input_len)

    return prompt, system_instructions, input_len
