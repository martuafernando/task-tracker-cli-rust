use std::error::Error;

use super::entity::{TaskEntity, TaskStatus};

pub trait TaskService {
    fn add(&mut self, taskname: &String) -> Result<i8, Box<dyn Error>>;
    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<TaskEntity>, Box<dyn Error>>;
    fn update_status(&mut self, id: i8, status: TaskStatus) -> Result<i8, Box<dyn Error>>;
    fn update_name(&mut self, id: i8, name: &String) -> Result<i8, Box<dyn Error>>;
    fn delete(&mut self, id: i8) -> Result<i8, Box<dyn Error>>;
}
