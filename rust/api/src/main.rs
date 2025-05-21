use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use std::error::Error;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    log::info!("server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> (StatusCode, String) {
   (StatusCode::OK, String::from("Hello from rust"))
}

async fn create_user( Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 0,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}