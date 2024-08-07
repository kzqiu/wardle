pub mod routes;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conflict {
    pub id: u32,
    pub name: String,
    pub start_date: i32,
    pub end_date: i32,
    pub casualties_min: u32,
    pub casualties_max: u32,
    pub belligerents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConflictList {
    pub conflicts: Vec<Conflict>,
}

pub struct ServerState {
    pub conflict_list: Arc<RwLock<ConflictList>>,
    pub target_conflict: Arc<RwLock<Conflict>>,
}

pub fn rand_conflict(conflict_list: &ConflictList) -> Conflict {
    let num = rand::thread_rng().gen_range(0..conflict_list.conflicts.len());
    conflict_list.conflicts[num].clone()
}

impl ServerState {
    pub fn new() -> Self {
        let conflict_list: ConflictList = serde_json::from_str(include_str!("data/conflicts.json"))
            .expect("Conflicts file was not well-formatted");

        let target_conflict = rand_conflict(&conflict_list);

        println!("Setting conflict to: {:#?}", target_conflict);

        ServerState {
            conflict_list: Arc::new(RwLock::new(conflict_list)),
            target_conflict: Arc::new(RwLock::new(target_conflict)),
        }
    }

    pub fn set_conflict(&self) {
        let read = self.conflict_list.read().unwrap();
        let mut write = self.target_conflict.write().unwrap();
        *write = rand_conflict(&read);
    }
}
