import os
from google import genai

api_key = os.environ.get("GEMINI_API_KEY")
client = genai.Client(api_key=api_key)


async def generate_content(prompt, system_instructions):
    response = await client.aio.models.generate_content(
        model="gemini-2.0-flash-exp",
        contents=prompt,
        config=genai.types.GenerateContentConfig(
            system_instruction=system_instructions,
        ),
    )
    return response.text
