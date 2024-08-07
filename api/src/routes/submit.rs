use crate::ServerState;
use axum::{
    extract::{self, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Answer {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct Response {
    pub correct: bool,
}

pub async fn submit(
    State(state): State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<Answer>,
) -> Json<Response> {
    let guess = (*state.conflict_list.read().unwrap()).conflicts[payload.id as usize].clone();
    let target = (*state.target_conflict).read().unwrap();

    let resp = Response {
        correct: guess.id == target.id,
    };

    Json(resp)
}
