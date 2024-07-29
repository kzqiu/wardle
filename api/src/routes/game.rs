use axum::{
    extract::{self, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conflict {
    id: i64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConflictList {
    conflicts: Vec<Conflict>,
}

pub async fn conflicts(State(conflict_list): State<Arc<ConflictList>>) -> Json<Arc<ConflictList>> {
    Json(conflict_list)
}

#[derive(Deserialize)]
pub struct Answer {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct Response {}

pub async fn submit(extract::Json(_payload): extract::Json<Answer>) -> Json<Response> {
    // payload is a struct Answer
    let resp = Response {};

    Json(resp)
}
