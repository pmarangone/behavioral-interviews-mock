use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};

#[derive(TryFromMultipart)]
pub struct IncomingRequest {
    #[form_data(limit = "25MiB")]
    pub file: FieldData<Bytes>,
    pub language: String,
    pub position: String,
    pub description: String,
    pub question_id: usize,
}
