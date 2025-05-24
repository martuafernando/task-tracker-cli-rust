use std::{env, process::exit};

mod domains;
mod applications;
mod infrastructures;
mod interfaces;

use infrastructures::storage::file_storage::FileStorage;
use infrastructures::repositories::task_repository_impl::TaskRepositoryImpl;
use applications::services::task_service_impl::TaskServiceImpl;
use interfaces::cli::task::{add_task, delete, list_task, mark_done, mark_in_progress, update};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!(
            "No command provided. Available commands: add, update, delete, mark-in-progress, mark-done, list."
        );
        exit(1);
    }

    let command: &String = &args[1];

    let storage = FileStorage::new("./data.json".to_string());
    let repository = TaskRepositoryImpl::new(storage);
    let mut service = TaskServiceImpl::new(repository);

    match command.as_str() {
        "add" => add_task(&mut service, &args),
        "list" => list_task(&mut service, &args),
        "mark-in-progress" => mark_in_progress(&mut service, &args),
        "mark-done" => mark_done(&mut service, &args),
        "update" => update(&mut service, &args),
        "delete" => delete(&mut service, &args),
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}