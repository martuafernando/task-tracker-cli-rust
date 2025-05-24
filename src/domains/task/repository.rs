use std::error::Error;

use super::model::Task;
use super::status::TaskStatus;

pub trait TaskRepository {
    fn create(&mut self, name: &String, status: TaskStatus) -> Result<i8, Box<dyn Error>>;
    fn get_all(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>>;
    fn get_by_id(&self, id: i8) -> Result<Task, Box<dyn Error>>;
    fn update(&mut self, id: i8, task: &Task) -> Result<i8, Box<dyn Error>>;
    fn delete(&mut self, id: i8) -> Result<i8, Box<dyn Error>>;
}
