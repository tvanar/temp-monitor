// router
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    let routes = Router::new().route("/", get(None));

    Router::new().merge(routes)
}
