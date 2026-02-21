use tracing::{error, info};

use crate::{
    models::{incoming::IncomingRequest, state::SharedState, task::TaskStatus},
    services::groq::{generate_response, transcribe_audio},
    state::update_status,
};

/// Processes an incoming task: transcribes audio, generates a response, and updates task status.
pub async fn process(task_id: String, data: IncomingRequest, state: SharedState) {
    update_status(&state, task_id.clone(), TaskStatus::Transcribing).await;

    let audio_contents = data.file.contents;
    let language = data.language;
    let position = data.position;
    let description = data.description;
    let question_id = data.question_id;

    match transcribe_audio(&state, audio_contents).await {
        Ok(text) => {
            update_status(&state, task_id.clone(), TaskStatus::Analyzing).await;

            info!("{}", text);

            match generate_response(
                &text,
                question_id,
                &language,
                &position,
                &description,
                &state,
            )
            .await
            {
                Ok(feedback) => {
                    state.tasks.insert(
                        task_id,
                        TaskStatus::Finished {
                            transcription: text,
                            feedback,
                        },
                    );
                }
                Err(e) => {
                    error!("{}", e);
                    state
                        .tasks
                        .insert(task_id, TaskStatus::Error(e.to_string()));
                }
            }
        }
        Err(e) => {
            error!("{}", e);
            state
                .tasks
                .insert(task_id, TaskStatus::Error(e.to_string()));
        }
    }
}
