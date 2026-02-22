use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QuestionsQuery {
    pub lang: Option<String>,
}

#[derive(Serialize)]
pub struct QuestionResponse {
    pub id: usize,
    pub question_en: String,
    pub question_it: String,
    pub question_pt: String,
    pub question_de: String,
}
