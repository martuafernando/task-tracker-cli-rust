use super::status::TaskStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub(crate) id: i8,
    pub(crate) name: String,
    pub(crate) status: TaskStatus,
}
