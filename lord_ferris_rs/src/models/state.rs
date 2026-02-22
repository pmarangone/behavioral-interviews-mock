use crate::models::task::TaskStatus;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub tasks: DashMap<String, TaskStatus>,
    pub config: crate::config::AppConfig,
    pub user_hits: DashMap<String, Vec<DateTime<Utc>>>,
    pub total_hits: DashMap<String, Vec<DateTime<Utc>>>,
    pub retry_after_transcription: RwLock<Option<DateTime<Utc>>>,
    pub retry_after_text_generation: RwLock<Option<DateTime<Utc>>>,
}

pub type SharedState = Arc<AppState>;
