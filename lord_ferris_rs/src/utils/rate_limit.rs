use crate::models::state::SharedState;
use crate::utils::constants::{
    DEFAULT_RETRY_AFTER, GLOBAL_DAILY_LIMIT, USER_DAY_LIMIT, USER_HOUR_LIMIT, USER_MIN_LIMIT,
};
use axum::http::HeaderMap;
use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use tokio::sync::RwLock;

pub fn check_global_rate_limit(state: &SharedState) -> Result<(), (StatusCode, String)> {
    let now = Utc::now();
    let mut total_hits = state
        .total_hits
        .entry("global".to_string())
        .or_insert(Vec::new());
    total_hits.retain(|&t| t > now - Duration::days(1));
    if total_hits.len() >= GLOBAL_DAILY_LIMIT {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Global daily limit reached".to_string(),
        ));
    }
    total_hits.push(now);
    Ok(())
}

pub fn check_user_rate_limit(
    state: &SharedState,
    user_ip: &str,
) -> Result<(), (StatusCode, String)> {
    let now = Utc::now();
    let mut user_hits = state
        .user_hits
        .entry(user_ip.to_string())
        .or_insert(Vec::new());

    let hits_min = user_hits
        .iter()
        .filter(|&&t| t > now - Duration::minutes(1))
        .count();
    if hits_min >= USER_MIN_LIMIT {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Per-minute limit reached".to_string(),
        ));
    }

    let hits_hour = user_hits
        .iter()
        .filter(|&&t| t > now - Duration::hours(1))
        .count();
    if hits_hour >= USER_HOUR_LIMIT {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Per-hour limit reached".to_string(),
        ));
    }

    user_hits.retain(|&t| t > now - Duration::days(1));
    if user_hits.len() >= USER_DAY_LIMIT {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Per-day limit reached".to_string(),
        ));
    }

    user_hits.push(now);
    Ok(())
}

pub async fn check_groq_rate_limit(
    retry_after: &RwLock<Option<DateTime<Utc>>>,
) -> anyhow::Result<()> {
    let retry_after = retry_after.read().await;
    if let Some(wait_until) = *retry_after {
        if Utc::now() < wait_until {
            return Err(anyhow::anyhow!(
                "Groq is currently rate limited until {}",
                wait_until
            ));
        }
    }
    Ok(())
}

pub async fn validate_groq_too_many_requests(
    status: StatusCode,
    headers: &HeaderMap,
    retry_after: &RwLock<Option<DateTime<Utc>>>,
) -> anyhow::Result<()> {
    if status == StatusCode::TOO_MANY_REQUESTS {
        let mut retry_after_val = DEFAULT_RETRY_AFTER;
        if let Some(retry_after_header) = headers.get("retry-after") {
            if let Ok(val) = retry_after_header.to_str() {
                if let Ok(seconds) = val.parse::<i64>() {
                    retry_after_val = seconds;
                }
            }
        }

        let wait_until = Utc::now() + Duration::seconds(retry_after_val);
        let mut retry_after = retry_after.write().await;
        *retry_after = Some(wait_until);

        return Err(anyhow::anyhow!(
            "Groq rate limit reached. Retry after {} seconds",
            retry_after_val
        ));
    }

    Ok(())
}
