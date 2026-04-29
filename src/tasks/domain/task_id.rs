use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskId(u64);

impl TaskId {
    pub fn value(&self) -> u64 {
        self.0
    }
}
