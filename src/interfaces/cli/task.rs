use std::process::exit;

use crate::domains::task::{service::TaskService, status::TaskStatus};

pub(crate) fn add_task<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let task_name = args.get(2).unwrap_or_else(|| {
        eprintln!("Error: Task name is required for the 'add' command.");
        exit(1)
    });

    let id = service.add(task_name);
    match id {
        Ok(task_id) => println!("Task added successfully (ID: {})", task_id),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub(crate) fn list_task<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let status_filter = match args.get(2) {
        Some(status) => match status.as_str() {
            "todo" => Some(TaskStatus::Todo),
            "in-progress" => Some(TaskStatus::InProgress),
            "done" => Some(TaskStatus::Done),
            _ => {
                eprintln!("Invalid status filter. Available filters: todo, in-progress, done.");
                exit(1);
            }
        },
        None => None,
    };

    let tasks = service.get(status_filter);

    match tasks {
        Ok(tasks) => {
            for task in tasks {
                println!("{}", task.name);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub(crate) fn mark_in_progress<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let task_id_str = args.get(2).unwrap_or_else(|| {
        eprintln!("Error: Task ID is required for the 'mark-in-progress' command.");
        exit(1);
    });

    let task_id = task_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Error: Task ID must be a valid number.");
        exit(1);
    });

    let result = service.update_status(task_id, TaskStatus::InProgress);

    match result {
        Ok(id) => {
            println!("Task updated successfully (ID: {})", id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub(crate) fn mark_done<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let task_id_str = args.get(2).unwrap_or_else(|| {
        eprintln!("Error: Task ID is required for the 'mark-in-progress' command.");
        exit(1);
    });

    let task_id = task_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Error: Task ID must be a valid number.");
        exit(1);
    });

    let result = service.update_status(task_id, TaskStatus::Done);

    match result {
        Ok(id) => {
            println!("Task updated successfully (ID: {})", id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub(crate) fn update<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let task_id_str = args.get(2).unwrap_or_else(|| {
        eprintln!("Error: Task ID is required for the 'update' command.");
        exit(1);
    });

    let task_id = task_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Error: Task ID must be a valid number.");
        exit(1);
    });

    let task_name = args.get(3).unwrap_or_else(|| {
        eprintln!("Error: Task name is required for the 'update' command.");
        exit(1);
    });

    let result = service.update_name(task_id, task_name);

    match result {
        Ok(id) => {
            println!("Task updated successfully (ID: {})", id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub(crate) fn delete<T: TaskService>(service: &mut T, args: &Vec<String>) {
    let task_id_str = args.get(2).unwrap_or_else(|| {
        eprintln!("Error: Task ID is required for the 'mark-in-progress' command.");
        exit(1);
    });

    let task_id = task_id_str.parse().unwrap_or_else(|_| {
        eprintln!("Error: Task ID must be a valid number.");
        exit(1);
    });

    let result = service.delete(task_id);

    match result {
        Ok(id) => {
            println!("Task deleted successfully (ID: {})", id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
