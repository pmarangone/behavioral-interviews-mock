pub const API_KEY_HEADER: &str = "custom_api_key";

pub const GLOBAL_DAILY_LIMIT: usize = 105;
pub const USER_MIN_LIMIT: usize = 2;
pub const USER_HOUR_LIMIT: usize = 10;
pub const USER_DAY_LIMIT: usize = 15;

pub const GROQ_WHISPER_URL: &str = "https://api.groq.com/openai/v1/audio/transcriptions";
pub const GROQ_WHISPER_MODEL: &str = "whisper-large-v3-turbo";
pub const GROQ_LLAMA_URL: &str = "https://api.groq.com/openai/v1/responses";
pub const GROQ_LLAMA_MODEL: &str = "llama-3.3-70b-versatile";
pub const DEFAULT_RETRY_AFTER: i64 = 60;

pub const GEMINI_MODEL: &str = "gemini-2.5-flash-lite";

pub const DEFAULT_LANGUAGE: &str = "en";
