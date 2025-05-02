use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            name,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true
    }
}
