use std::collections::{HashMap, VecDeque};

use crate::{
    errors::AppError,
    models::{Stats, TempPoint},
    state::SharedState,
    templates::PageTemplate,
};

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Html,
};

use askama::Template;
use chrono::Utc;

const MAX_TEMP_POLLS: usize = 1440;

pub async fn display_stats_page() -> Result<Html<String>, AppError> {
    let template = PageTemplate;

    let html = template.render().map_err(|e| {
        eprintln!("Template error: {e}");
        AppError::TemplateError
    })?;

    Ok(Html(html))
}

pub async fn add_data_handler(
    State(state): State<SharedState>,
    Json(stats): Json<Stats>,
) -> StatusCode {
    // get a reference to the hostname (use clone)
    let hostname = stats.hostname.clone();

    let mut state = state.write().unwrap();

    let temp_now = TempPoint {
        time: Utc::now(),
        temp: stats.temp,
    };

    if let Some(temp_vec) = state.temp.get_mut(&hostname) {
        temp_vec.push_back(temp_now);
        if temp_vec.len() > MAX_TEMP_POLLS {
            temp_vec.pop_front();
        }
    } else {
        let mut vec = VecDeque::new();
        vec.push_back(temp_now);
        state.temp.insert(hostname.clone(), vec);
    }

    state.current_stats.insert(hostname.clone(), stats);

    state.last_seen.insert(hostname, Utc::now());

    StatusCode::OK
}

pub async fn publish_stats(State(state): State<SharedState>) -> Json<Vec<Stats>> {
    let stats = state.read().unwrap();
    let data: Vec<Stats> = stats.current_stats.values().cloned().collect();
    Json(data)
}

pub async fn publish_temps(
    State(state): State<SharedState>,
) -> Json<HashMap<String, VecDeque<TempPoint>>> {
    let stats = state.read().unwrap();
    Json(stats.temp.clone())
}
