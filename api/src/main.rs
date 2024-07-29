use api::routes::{
    game::{self, ConflictList},
    health, session,
};
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let conflict_list: ConflictList = serde_json::from_str(include_str!("data/conflicts.json"))
        .expect("Conflicts file was not well-formatted");

    let state = Arc::new(conflict_list);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/session", post(session::session))
        .route("/game/conflicts", get(game::conflicts))
        .route("/game/submit", post(game::submit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
