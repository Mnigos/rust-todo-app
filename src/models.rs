use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub is_completed: bool,
}
