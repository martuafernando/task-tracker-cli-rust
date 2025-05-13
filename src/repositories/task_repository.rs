use crate::models::task::Task;
use crate::models::task_status::TaskStatus;
use crate::storage::file_storage::Storage;
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

    fn get_new_id(&self, tasks: &[Task]) -> i8 {
        tasks
            .iter()
            .max_by_key(|taks| taks.id)
            .map(|task| task.id + 1)
            .unwrap_or(1)
    }
}

impl<S: Storage> TaskRepository for TaskRepositoryImpl<S> {
    fn create(&mut self, name: &String, status: TaskStatus) -> Result<i8, Box<dyn Error>> {
        let mut data = self.storage.load()?;
        let new_id = self.get_new_id(&data);

        let task = Task {
            id: new_id,
            name: name.clone(),
            status: status,
        };

        let task_id = task.id;

        data.push(task);
        self.storage.save(&data)?;

        Ok(task_id)
    }

    fn get(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>> {
        let tasks = self.storage.load()?;

        match status {
            None => Ok(tasks),
            Some(status_filter) => Ok(tasks
                .into_iter()
                .filter(|task| task.status == status_filter)
                .collect()),
        }
    }
}
