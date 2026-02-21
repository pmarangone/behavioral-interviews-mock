#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status", content = "data")]
pub enum TaskStatus {
    Pending,
    Transcribing,
    Analyzing,
    Finished {
        transcription: String,
        feedback: String,
    },
    Error(String),
}
