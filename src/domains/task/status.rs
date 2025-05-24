use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
  Todo,
  InProgress,
  Done,
}