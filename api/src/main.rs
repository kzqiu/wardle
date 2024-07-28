use axum::{routing::get, routing::post, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/submit", post(submit_answer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn root() {}

async fn health_check() -> String {
    String::from("Hello world!")
}

async fn submit_answer() {}
