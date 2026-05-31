use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// router
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{add_data_handler, display_stats_page, publish_stats, publish_temps},
    state::AppState,
};

pub fn create_router() -> Router {
    let current_stats = HashMap::new();
    let temp = HashMap::new();

    let state = Arc::new(RwLock::new(AppState {
        current_stats,
        temp,
    }));

    let routes = Router::new()
        .route("/", get(display_stats_page))
        .route("/api/stats", get(publish_stats))
        .route("/api/temps", get(publish_temps))
        .route("/push-data", post(add_data_handler));

    Router::new().merge(routes).with_state(state)
}
