use api::routes::{game, health, session};
use api::{Conflict, ConflictList, ServerState};
use axum::{routing::get, routing::post, Router};
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    // Set up app state, composed of:
    // 1. list of conflicts,
    // 2. target conflict.
    let conflict_list: Arc<RwLock<ConflictList>> =
        serde_json::from_str(include_str!("data/conflicts.json"))
            .expect("Conflicts file was not well-formatted");

    let target_conflict = Arc::new(RwLock::new(Conflict {
        id: 0,
        name: String::from("test"),
    }));

    let state = Arc::new(ServerState {
        conflict_list,
        target_conflict,
    });

    // Start router and begin listening.
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/session", post(session::session))
        .route("/game/conflicts", get(game::conflicts))
        .route("/game/submit", post(game::submit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
