pub mod routes;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conflict {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConflictList {
    pub conflicts: Vec<Conflict>,
}

pub struct ServerState {
    pub conflict_list: Arc<RwLock<ConflictList>>,
    pub target_conflict: Arc<RwLock<Conflict>>,
}
