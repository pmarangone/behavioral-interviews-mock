use crate::models::{state::SharedState, task::TaskStatus};
use tracing::info;

pub async fn update_status(state: &SharedState, id: String, status: TaskStatus) {
    state.tasks.insert(id, status);
}

pub async fn check_status(state: &SharedState, id: String) {
    if let Some(status) = state.tasks.get(&id) {
        info!("Status is: {:?}", *status);
    }
}
