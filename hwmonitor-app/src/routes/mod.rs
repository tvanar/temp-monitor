use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    Router,
    routing::{get, post},
};
use chrono::Utc;

use std::time::Duration;

use tokio::time::sleep;

use crate::{
    handlers::{add_data_handler, display_stats_page, publish_stats, publish_temps},
    state::AppState,
};

pub fn create_router() -> Router {
    let current_stats = HashMap::new();
    let temp = HashMap::new();
    let last_seen = HashMap::new();

    let state = Arc::new(RwLock::new(AppState {
        current_stats,
        temp,
        last_seen,
    }));

    let cleanup_state = state.clone();
    tokio::spawn(async move {
        let timeout_secs = 300;
        let check_interval = Duration::from_secs(5);
        loop {
            sleep(check_interval).await;
            let mut state = cleanup_state.write().unwrap();
            let now = Utc::now();
            let mut stale = Vec::new();
            for (host, last) in &state.last_seen {
                if (now - *last).num_seconds() > timeout_secs {
                    stale.push(host.clone());
                }
            }
            for host in &stale {
                state.current_stats.remove(host);
                state.temp.remove(host);
                state.last_seen.remove(host);
            }
        }
    });

    let routes = Router::new()
        .route("/", get(display_stats_page))
        .route("/api/stats", get(publish_stats))
        .route("/api/temps", get(publish_temps))
        .route("/push-data", post(add_data_handler));

    Router::new().merge(routes).with_state(state)
}
