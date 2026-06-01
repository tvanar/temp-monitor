use axum;
use std::net::SocketAddr;
mod errors;
mod handlers;
mod models;
mod routes;
mod state;
mod templates;

fn get_server_config() -> SocketAddr {
    let _ = dotenvy::dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Parse the full string "127.0.0.1:3001"
    let addr_str = format!("{}:{}", host, port);
    let addr: SocketAddr = addr_str
        .parse()
        .expect("Could not parse HOST and PORT into a valid SocketAddr");

    addr
}

#[tokio::main]
async fn main() {
    let addr = get_server_config();

    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
