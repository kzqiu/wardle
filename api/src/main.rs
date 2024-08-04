use api::routes::{game, health, session};
use api::ServerState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    // Set up app state, composed of:
    // 1. list of conflicts,
    // 2. target conflict.
    let state = Arc::new(ServerState::new());

    // Start job to change target_conflict every day.
    let sched = JobScheduler::new().await.unwrap();
    sched
        .add(
            // UTC-7 is west coast time
            {
                let s = state.clone();
                // Testing: Job::new("1/10 * * * * *", move |_uuid, _l| {
                Job::new("0 0 7 * * * *", move |_uuid, _l| {
                    s.set_conflict();
                    println!(
                        "New target conflict: {:?}",
                        s.target_conflict.read().unwrap()
                    );
                })
                .unwrap()
            },
        )
        .await
        .unwrap();

    sched.start().await.unwrap();

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
