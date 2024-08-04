pub mod routes;
use rand::Rng;
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

impl ServerState {
    pub fn new() -> Self {
        let conflict_list: Arc<RwLock<ConflictList>> =
            serde_json::from_str(include_str!("data/conflicts.json"))
                .expect("Conflicts file was not well-formatted");

        let target_conflict = Arc::new(RwLock::new(Conflict {
            id: 0,
            name: String::from("test"),
        }));

        ServerState {
            conflict_list,
            target_conflict,
        }
    }

    pub fn set_conflict(&self) {
        let read = self.conflict_list.read().unwrap();
        let num = rand::thread_rng().gen_range(0..(*read.conflicts).len());
        let conflict = (*read).conflicts[num].clone();

        let mut write = self.target_conflict.write().unwrap();
        *write = conflict;
    }
}
