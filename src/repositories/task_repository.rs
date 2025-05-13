use crate::models::task::Task;
use crate::models::task_status::TaskStatus;
use crate::storage::file_storage::Storage;
use std::error::Error;
use std::task;
pub trait TaskRepository {
    fn create(&mut self, name: &String, status: TaskStatus) -> Result<i8, Box<dyn Error>>;
    fn get_all(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>>;
    fn get_by_id(&self, id: i8) -> Result<Task, Box<dyn Error>>;
    fn update(&mut self, id: i8, task: &Task) -> Result<i8, Box<dyn Error>>;
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

    fn get_all(&self, status: Option<TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>> {
        let tasks = self.storage.load()?;

        match status {
            None => Ok(tasks),
            Some(status_filter) => Ok(tasks
                .into_iter()
                .filter(|task| task.status == status_filter)
                .collect()),
        }
    }

    fn get_by_id(&self, id: i8) -> Result<Task, Box<dyn Error>> {
        let tasks = self.storage.load()?;

        let result = tasks.iter().find(|tasks| tasks.id == id);

        match result {
            None => Err(format!("Task with ID {} not found", id).into()),
            Some(task) => Ok(task.clone()),
        }
    }

    fn update(&mut self, id: i8, task: &Task) -> Result<i8, Box<dyn Error>> {
        let mut tasks = self.storage.load()?;

        if let Some(existing_task) = tasks.iter_mut().find(|t| t.id == id) {
            existing_task.name = task.name.clone();
            existing_task.status = task.status.clone();
        } else {
            return Err(format!("Task with ID {} not found", id).into());
        }

        self.storage.save(&tasks)?;

        Ok(id)
    }
}
