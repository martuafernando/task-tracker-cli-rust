use mockall::*;
use mockall::predicate::*;
use std::error::Error;

use crate::domains::task::entity::TaskEntity;

#[automock]
pub trait Storage {
    fn save(&self, tasks: &[TaskEntity]) -> Result<(), Box<dyn Error>>;
    fn load(&self) -> Result<Vec<TaskEntity>, Box<dyn Error>>;
}
