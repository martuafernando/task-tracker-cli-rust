use std::error::Error;

use crate::domains::task::entity::{TaskEntity, TaskStatus};
use crate::domains::task::repository::TaskRepository;
use crate::domains::task::service::TaskService;

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
        self.repository.create(taskname, TaskStatus::Todo)
    }

    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<TaskEntity>, Box<dyn Error>> {
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
        self.repository.get_by_id(id)?;
        self.repository.delete(id)
    }
}
