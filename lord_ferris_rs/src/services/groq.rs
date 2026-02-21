use crate::models::groq::{GroqResponse, GroqTextResponse};
use crate::models::state::SharedState;
use crate::utils::constants::{
    GROQ_LLAMA_MODEL, GROQ_LLAMA_URL, GROQ_WHISPER_MODEL, GROQ_WHISPER_URL,
};
use crate::utils::rate_limit::{check_groq_rate_limit, validate_groq_too_many_requests};
use anyhow::Result;
use axum::body::Bytes;
use reqwest::{Client, multipart};

/// Transcribes audio bytes using Groq Whisper model.
pub async fn transcribe_audio(state: &SharedState, audio_bytes: Bytes) -> Result<String> {
    check_groq_rate_limit(&state.retry_after_transcription).await?;

    let client = Client::new();

    let part = multipart::Part::bytes(audio_bytes.to_vec())
        .file_name("audio.wav")
        .mime_str("audio/wav")?;

    let form = multipart::Form::new()
        .part("file", part)
        .text("model", GROQ_WHISPER_MODEL)
        .text("response_format", "json");

    let response = client
        .post(GROQ_WHISPER_URL)
        .header(
            "Authorization",
            format!("Bearer {}", state.config.groq_api_key),
        )
        .multipart(form)
        .send()
        .await?;

    let status = response.status();
    let headers = response.headers();
    validate_groq_too_many_requests(status, headers, &state.retry_after_transcription).await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Groq Error: {}", response.status()));
    }

    let data = response.json::<GroqTextResponse>().await?;

    Ok(data.text)
}

/// Generates a response using Groq Llama model based on the provided transcription and context.
pub async fn generate_response(
    transcription_text: &str,
    question_id: usize,
    language: &str,
    position: &str,
    description: &str,
    state: &SharedState,
) -> Result<String, anyhow::Error> {
    check_groq_rate_limit(&state.retry_after_text_generation).await?;

    let client = Client::new();
    let prompt = crate::prompts::get_prompt(
        transcription_text,
        question_id,
        language,
        position,
        description,
    );

    let body = serde_json::json!({
        "input": prompt,
        "model": GROQ_LLAMA_MODEL,
    });

    let response = client
        .post(GROQ_LLAMA_URL)
        .header(
            "Authorization",
            format!("Bearer {}", state.config.groq_api_key),
        )
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let headers = response.headers();
    validate_groq_too_many_requests(status, headers, &state.retry_after_text_generation).await?;

    let raw_body = response.text().await?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Groq Error: {} - Body: {}",
            status,
            raw_body
        ));
    }

    let data = serde_json::from_str::<GroqResponse>(&raw_body)?;
    let content = data
        .output
        .into_iter()
        .find_map(|output_item| {
            if output_item.output_type == "message" {
                output_item.content.and_then(|content_vec| {
                    content_vec.into_iter().find_map(|content_item| {
                        if content_item.content_type == "output_text" {
                            Some(content_item.text)
                        } else {
                            None
                        }
                    })
                })
            } else {
                None
            }
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No 'message' type output with 'output_text' content found in Groq response"
            )
        })?;

    Ok(content)
}
