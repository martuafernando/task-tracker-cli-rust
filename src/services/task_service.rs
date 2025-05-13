use std::error::Error;

use crate::models::task::Task;
use crate::models::task_status::TaskStatus;
use crate::repositories::task_repository::TaskRepository;

pub trait TaskService {
    fn add(&mut self, taskname: &String) -> Result<i8, Box<dyn Error>>;
    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>>;
    fn update_status(&mut self, id: i8, status: TaskStatus) -> Result<i8, Box<dyn Error>>;
    fn update_name(&mut self, id: i8, name: &String) -> Result<i8, Box<dyn Error>>;
    fn delete(&mut self, id: i8) -> Result<i8, Box<dyn Error>>;
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
        self.repository.get_all(status)
    }
    
    fn update_status(&mut self, id: i8, status: TaskStatus) -> Result<i8, Box<dyn Error>> {
        let mut task = self.repository.get_by_id(id)?;
        task.status = status;

        self.repository.update(id, &task)       
    }
    
    fn update_name(&mut self, id: i8, name: &String) -> Result<i8, Box<dyn Error>> {
        let mut task = self.repository.get_by_id(id)?;
        task.name = name.to_string();

        self.repository.update(id, &task)
    }
    
    fn delete(&mut self, id: i8) -> Result<i8, Box<dyn Error>> {
        let mut task = self.repository.get_by_id(id)?;

        self.repository.delete(id)
    }
}
