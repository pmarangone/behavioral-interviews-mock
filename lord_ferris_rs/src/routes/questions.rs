use crate::models::questions::{QuestionResponse, QuestionsQuery};
use crate::services::questions::fetch_random_question;
use crate::utils::constants::DEFAULT_LANGUAGE;
use axum::{Json, extract::Query};

pub async fn get_questions(Query(query): Query<QuestionsQuery>) -> Json<Vec<QuestionResponse>> {
    let lang = query
        .lang
        .clone()
        .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string());
    Json(fetch_random_question(&lang))
}
