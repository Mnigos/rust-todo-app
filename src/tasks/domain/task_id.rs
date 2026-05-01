use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
