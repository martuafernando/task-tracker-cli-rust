use std::error::Error;

use crate::domains::task::model::Task;

pub trait Storage {
    fn save(&self, tasks: &[Task]) -> Result<(), Box<dyn Error>>;
    fn load(&self) -> Result<Vec<Task>, Box<dyn Error>>;
}
