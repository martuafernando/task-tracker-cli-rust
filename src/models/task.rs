use super::task_status::TaskStatus;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub(crate) id: i8,
    pub(crate) name: String,
    pub(crate) status: TaskStatus,
}