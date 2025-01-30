from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor
from transformers.pipelines import pipeline
import torch
import soundfile as sf
from pydub import AudioSegment
import io

from app.api.response import server_error, success
from app.utils.logger import get_logger
from app.utils.save_user_input import upload_all

MAX_FILE_SIZE_MB = 5

logger = get_logger(__name__)


class WhisperModelSingleton:
    _instance = None
    model_id = "openai/whisper-large-v3-turbo"
    torch_dtype = torch.float16 if torch.cuda.is_available() else torch.float32
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    def __new__(cls, *args, **kwargs):
        if not isinstance(cls._instance, cls):
            cls._instance = super(WhisperModelSingleton, cls).__new__(
                cls, *args, **kwargs
            )
            cls._instance.init_model()
        return cls._instance

    def init_model(self):
        self.model = AutoModelForSpeechSeq2Seq.from_pretrained(
            self.model_id,
            torch_dtype=self.torch_dtype,
            low_cpu_mem_usage=False,  # Pay attention to this
            use_safetensors=True,
        )
        self.model.generation_config.return_timestamps = True
        self.model.to(self.device)

        self.processor = AutoProcessor.from_pretrained(self.model_id)
        self.pipe = pipeline(
            "automatic-speech-recognition",
            model=self.model,
            tokenizer=self.processor.tokenizer,
            feature_extractor=self.processor.feature_extractor,
            torch_dtype=self.torch_dtype,
            device=self.device,
        )

    def get_result(self, audio_data, sample_rate, language):
        result = self.pipe(
            {"sampling_rate": sample_rate, "raw": audio_data},
            generate_kwargs={"language": language},
        )
        return result

    def process_file(self, file_bytes):
        audio_file = io.BytesIO(file_bytes)

        audio = AudioSegment.from_file(audio_file, format="webm")

        wav_buffer = io.BytesIO()
        audio.export(wav_buffer, format="wav")
        wav_buffer.seek(0)

        audio_data, sample_rate = sf.read(wav_buffer)

        # Ensure audio is mono
        if len(audio_data.shape) > 1:
            audio_data = audio_data.mean(axis=1)

        return audio_data, sample_rate

    async def output(self, file, save, language="english"):
        try:
            logger.info("File size %s", file.size)

            if file.size > MAX_FILE_SIZE_MB * 1024 * 1024:
                err = f"File size is greater than {MAX_FILE_SIZE_MB}"
                logger.error(err)
                return server_error({"error": err})

            file_bytes = await file.read()

            audio, sample = self.process_file(file_bytes)

            output = self.get_result(audio, sample, language)

            transcript = output["text"]

            """ Probably Pub/Sub if using a db or handling multiple users
                Store the glob data to something like AWS S3
            """
            # if save:
            #     logger.info("Sending file to be stored.")
            #     upload_all(save, file, transcript)
            # else:
            #     logger.info("Storing file failed.")

            return success({"transcript": transcript})

        except Exception as ex:
            error = str(ex)
            logger.error(f"Error while transcribing input: {error}")
            return server_error({"error": error})
