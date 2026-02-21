use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method},
    routing::{get, post},
};
use dashmap::DashMap;
use lord_ferris_rs::config::AppConfig;
use lord_ferris_rs::models::state::AppState;
use lord_ferris_rs::routes::health::healthz;
use lord_ferris_rs::routes::incoming::incoming;
use lord_ferris_rs::routes::questions::get_questions;
use lord_ferris_rs::routes::sse::sse_handler;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{error, info};

use lord_ferris_rs::utils::constants::API_KEY_HEADER;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = AppConfig::new().expect("Failed to load configuration");
    let server_port = config.server_port.clone();
    let custom_api_key_value =
        HeaderValue::from_str(&config.api_key_value).expect("Invalid API key value in config");
    let vercel_domain = config.vercel_domain.clone();
    let state = Arc::new(AppState {
        tasks: DashMap::new(),
        config: config.clone(),
        user_hits: DashMap::new(),
        total_hits: DashMap::new(),
        retry_after_transcription: tokio::sync::RwLock::new(None),
        retry_after_text_generation: tokio::sync::RwLock::new(None),
    });

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, request_parts: &axum::http::request::Parts| {
                let origin_str = origin.to_str().unwrap_or("");

                let is_origin_allowed = if let Ok(origin_url) = url::Url::parse(origin_str) {
                    if let Some(host) = origin_url.host_str() {
                        info!(host);
                        let is_vercel_https =
                            host == vercel_domain && origin_url.scheme() == "https";
                        let is_localhost_http =
                            (host == "localhost" || host == "127.0.0.1" || host == "0.0.0.0")
                                && origin_url.scheme() == "http";
                        is_vercel_https || is_localhost_http
                    } else {
                        false
                    }
                } else {
                    false
                };

                let has_valid_api_key = request_parts
                    .headers
                    .get(API_KEY_HEADER)
                    .map_or(false, |header_value| header_value == custom_api_key_value);

                is_origin_allowed || has_valid_api_key
            },
        ))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ORIGIN,
            HeaderName::from_static(API_KEY_HEADER),
        ]);

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/stream/{id}", get(sse_handler))
        .route("/transcribe", post(incoming))
        .route("/questions", get(get_questions).post(get_questions))
        .layer(cors)
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port)).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind to port {}: {:?}", server_port, err);
            return;
        }
    };

    if let Err(err) = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    {
        error!("Failed to start server: {:?}", err);
    }
}
