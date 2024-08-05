use axum::{extract, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Answer {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct Response {}

pub async fn submit(extract::Json(_payload): extract::Json<Answer>) -> Json<Response> {
    // payload is a struct Answer
    let resp = Response {};

    Json(resp)
}
