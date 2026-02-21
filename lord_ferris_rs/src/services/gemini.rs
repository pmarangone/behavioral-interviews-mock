use anyhow::{Error, Result};
use google_cloud_aiplatform_v1::{
    client::PredictionService,
    model::{Content, GenerationConfig, Part, generation_config::ThinkingConfig, part::Data},
};

use crate::prompts::get_prompt;
use crate::utils::constants::GEMINI_MODEL;

fn extract_text_from_candidates(
    candidates: Vec<google_cloud_aiplatform_v1::model::Candidate>,
) -> Result<String, Error> {
    candidates
        .into_iter()
        .find_map(|candidate| {
            let text = candidate
                .content?
                .parts
                .into_iter()
                .filter_map(|part| match part.data? {
                    Data::Text(t) => Some(t),
                    _ => None,
                })
                .collect::<String>();

            if text.is_empty() { None } else { Some(text) }
        })
        .ok_or_else(|| Error::msg("No text content found in any candidate"))
}

/// Generates a response using Google Gemini model based on the provided transcription and context.
pub async fn generate_response(
    transcription_text: &str,
    question_id: usize,
    language: &str,
    position: &str,
    description: &str,
    project_id: &str,
) -> Result<String, Error> {
    let client = PredictionService::builder().build().await?;

    let model =
        format!("projects/{project_id}/locations/global/publishers/google/models/{GEMINI_MODEL}");
    let prompt = get_prompt(
        transcription_text,
        question_id,
        language,
        position,
        description,
    );

    let thinking_config = ThinkingConfig::new().set_thinking_budget(0i32);

    let generation_config = GenerationConfig::new()
        .set_response_mime_type("text/plain")
        .set_thinking_config(thinking_config);

    let response = client
        .generate_content()
        .set_model(&model)
        .set_contents([Content::new()
            .set_role("user")
            .set_parts([Part::new().set_text(prompt)])])
        .set_generation_config(generation_config)
        .send()
        .await?;

    extract_text_from_candidates(response.candidates)
}
