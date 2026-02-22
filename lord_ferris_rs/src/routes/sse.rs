use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};

use crate::models::{state::SharedState, task::TaskStatus};

pub async fn sse_handler(
    Path(task_id): Path<String>,
    State(state): State<SharedState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::unfold((state, false), move |(state, is_finished)| {
        let task_id_cloned = task_id.clone();
        async move {
            if is_finished {
                return None;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;

            let current_status = state.tasks.get(&task_id_cloned).map(
                |ref_multi: dashmap::mapref::one::Ref<'_, String, TaskStatus>| (*ref_multi).clone(),
            );

            match current_status {
                Some(status) => {
                    let json_data = serde_json::to_string(&status).unwrap_or_default();
                    let event = Event::default().data(json_data);

                    match status {
                        TaskStatus::Finished { .. } | TaskStatus::Error(_) => {
                            state.tasks.remove(&task_id_cloned);
                            Some((Ok(event), (state, true)))
                        }
                        _ => Some((Ok(event), (state, false))),
                    }
                }
                None => {
                    let err = r#"{"status":"Error","data":"Task not found"}"#;
                    Some((Ok(Event::default().data(err)), (state, true)))
                }
            }
        }
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
