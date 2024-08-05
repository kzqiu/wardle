use crate::{ConflictList, ServerState};
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn conflicts(State(state): State<Arc<ServerState>>) -> Json<ConflictList> {
    Json(state.conflict_list.read().unwrap().clone())
}
