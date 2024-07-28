use api::routes::{health, session, submit_answer};
use axum::{routing::get, routing::post, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/session", post(session::session))
        .route("/submit", post(submit_answer::submit_answer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
