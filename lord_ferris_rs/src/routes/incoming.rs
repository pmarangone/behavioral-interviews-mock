use crate::{
    models::{incoming::IncomingRequest, state::SharedState},
    services::process::process,
    utils::rate_limit::{check_global_rate_limit, check_user_rate_limit},
};
use axum::{
    Json,
    extract::{ConnectInfo, State},
};
use axum_typed_multipart::TypedMultipart;
use reqwest::StatusCode;
use serde::Serialize;
use std::net::SocketAddr;

use tracing::info;
use uuid::Uuid;

#[derive(Serialize)]
pub struct IncomingResponse {
    pub task_id: String,
}

pub async fn incoming(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedMultipart(data): TypedMultipart<IncomingRequest>,
) -> Result<Json<IncomingResponse>, (StatusCode, String)> {
    let user_ip = addr.ip().to_string();

    check_global_rate_limit(&state)?;
    check_user_rate_limit(&state, &user_ip)?;

    let task_id = Uuid::new_v4().to_string();
    info!(
        "Received file for position: {} from IP: {}",
        data.position, user_ip
    );

    let task_id_for_spawn = task_id.clone();
    let state_for_spawn = state.clone();

    tokio::spawn(async move {
        process(task_id_for_spawn, data, state_for_spawn).await;
    });

    Ok(Json(IncomingResponse { task_id }))
}
