use crate::{ConflictList, ServerState};
use axum::{
    extract::{self, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn conflicts(State(state): State<Arc<ServerState>>) -> Json<ConflictList> {
    Json(state.conflict_list.read().unwrap().clone())
}

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
