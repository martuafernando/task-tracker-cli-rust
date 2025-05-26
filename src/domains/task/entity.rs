use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskEntity {
    pub(crate) id: i8,
    pub(crate) name: String,
    pub(crate) status: TaskStatus,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

mod tests {
    use super::*;

    #[test]
    fn test_task_serialization() {
        // Arrange
        let task = TaskEntity {
            id: 1,
            name: "Test Task".to_string(),
            status: TaskStatus::Done,
        };

        // Act
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: TaskEntity = serde_json::from_str(&json).unwrap();

        // Assert
        assert_eq!(task.id, deserialized.id);
        assert_eq!(task.name, deserialized.name);
        assert_eq!(task.status, deserialized.status);
    }

    #[test]
    fn test_task_clone() {
        // Arrange
        let task = TaskEntity {
            id: 1,
            name: "Test Task".to_string(),
            status: TaskStatus::Done,
        };

        // Act
        let cloned_task = task.clone();

        // Assert
        assert_eq!(task.id, cloned_task.id);
        assert_eq!(task.name, cloned_task.name);
        assert_eq!(task.status, cloned_task.status);
    }

    #[test]
    fn test_task_status_serialization() {
        // Arrange
        let task_status = TaskStatus::Done;

        // Act
        let json = serde_json::to_string(&task_status).unwrap();
        let deserialized: TaskStatus = serde_json::from_str(&json).unwrap();

        // Assert
        assert_eq!(task_status, deserialized);
    }

    #[test]
    fn test_task_status_clone() {
        // Arrange
        let task_status = TaskStatus::Done;

        // Act
        let cloned_task_status = task_status.clone();

        // Assert
        assert_eq!(task_status, cloned_task_status);
    }
}
