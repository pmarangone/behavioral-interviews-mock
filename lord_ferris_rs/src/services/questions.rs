use crate::models::questions::QuestionResponse;
use crate::utils::questions::{
    get_de_question, get_en_question, get_it_question, get_pt_question, get_random_question,
};

/// Fetches a random question in multiple languages based on the requested language.
pub fn fetch_random_question(lang: &str) -> Vec<QuestionResponse> {
    if let Some((id, _)) = get_random_question(lang) {
        vec![QuestionResponse {
            id,
            question_en: get_en_question(id).unwrap_or("").to_string(),
            question_it: get_it_question(id).unwrap_or("").to_string(),
            question_pt: get_pt_question(id).unwrap_or("").to_string(),
            question_de: get_de_question(id).unwrap_or("").to_string(),
        }]
    } else {
        vec![]
    }
}
