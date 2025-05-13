use std::error::Error;

use crate::models::task::Task;
use crate::models::task_status::TaskStatus;
use crate::repositories::task_repository::TaskRepository;

pub trait TaskService {
    fn add(&mut self, taskname: &String) -> Result<i8, Box<dyn Error>>;
    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>>;
}

pub struct TaskServiceImpl<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> TaskServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: TaskRepository> TaskService for TaskServiceImpl<R> {
    fn add(&mut self, taskname: &String) -> Result<i8, Box<dyn Error>> {
        self
            .repository
            .create(taskname, TaskStatus::InProgress)
    }
    
    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>> {
        self.repository.get(status)
    }
}
