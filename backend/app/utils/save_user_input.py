import os
from datetime import datetime

from app.utils.logger import get_logger

logger = get_logger(__name__)


def get_file_path(save_dir, file_type, timestamp=None):
    os.makedirs(save_dir, exist_ok=True)

    filename = f"{timestamp}.{file_type}"
    file_path = os.path.join(save_dir, filename)

    return file_path


def upload_all(upload, file=None, transcript=None):
    timestamp = datetime.now().strftime("%Y-%m-%d_%H-%M")

    audio_saved, text_saved = False, False

    match upload:
        case "all":
            audio_saved = upload_audio(file, timestamp)
            text_saved = upload_text(transcript, timestamp)
        case "audio":
            audio_saved = upload_audio(file, timestamp)
        case "text":
            text_saved = upload_text(transcript, timestamp)

    return audio_saved, text_saved


def upload_audio(file, timestamp):
    try:
        file_path = get_file_path("recordings", "ogg", timestamp)

        with open(file_path, "wb") as f:
            f.write(file)

        logger.info("Audio uploaded successfully.")
        return True

    except Exception as e:
        logger.error(f"Error uploading audio: {e}")
        return False


# Can be extended to upload the feedback response too
def upload_text(text, timestamp):
    try:
        file_path = get_file_path("transcripts", "txt", timestamp)

        with open(file_path, "w", encoding="utf-8") as f:
            f.write(text)

        logger.info("Text uploaded successfully.")
        return True

    except Exception as e:
        logger.error(f"Error uploading text: {e}")
        return False
