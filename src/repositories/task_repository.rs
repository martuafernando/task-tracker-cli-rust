use crate::models::task::Task;
use crate::models::task_status::TaskStatus;
use crate::storage::file_storage::Storage;
use std::collections::HashMap;
use std::error::Error;
pub trait TaskRepository {
    fn create(&mut self, name: &String, status: TaskStatus) -> Result<i8, Box<dyn Error>>;
    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>>;
}

pub struct TaskRepositoryImpl<S: Storage> {
    pub(crate) storage: S,
}

impl<S: Storage> TaskRepositoryImpl<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> TaskRepository for TaskRepositoryImpl<S> {
    fn create(&mut self, name: &String, status: TaskStatus) -> Result<i8, Box<dyn Error>> {
        let task = Task {
            id: 1,
            name: name.clone(),
            status: status,
        };
        
        
        let task_id = task.id;

        let mut data = self.storage.load()?;
        data.push(task);
        self.storage.save(&data)?;
        
        Ok(task_id)
    }

    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>> {
        let tasks = self.storage.load()?;
    
        match status {
            None => {
                Ok(tasks)
            },
            Some(status_filter) => {
                Ok(tasks
                    .into_iter()
                    .filter(|task| task.status == status_filter)
                    .collect())
            },
        }
    }
}
