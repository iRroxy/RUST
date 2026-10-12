use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
#[warn(unused_imports)]
#[warn(dead_code)]
#[warn(unused_variables)]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(contents) => serde_json::from_str(&contents).expect("failed to parse tasks.json"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => {
            panic!("failed to read tasks.json: {error}")
        }
    }
}
fn save_tasks(tasks: &[Task]) {
    let contents = serde_json::to_string_pretty(tasks).expect("failed to serialize tasks");

    fs::write("tasks.json", contents).expect("failed to write tasks.json");
}
fn parse_id(args: &[String]) -> Result<usize, String> {
    let raw_id = args
        .get(2)
        .ok_or_else(|| "please enter task id".to_string())?;

    raw_id
        .parse::<usize>()
        .map_err(|_| "task id must be a number".to_string())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            let title = match args.get(2) {
                Some(title) if !title.trim().is_empty() => title.to_string(),
                _ => {
                    eprintln!("error: please enter task contents");
                    return;
                }
            };

            let next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

            tasks.push(Task {
                id: next_id,
                title,
                completed: false,
            });
            save_tasks(&tasks);
            println!("add task: {}", next_id);
        }
        Some("list") => {
            for task in &tasks {
                let status = if task.completed { "[x]" } else { "[ ]" };
                println!("{} {} {}", task.id, status, task.title);
            }
        }
        Some("done") => {
            let id = match parse_id(&args) {
                Ok(id) => id,
                Err(message) => {
                    eprintln!("error: {message}");
                    return;
                }
            };

            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.completed = true;
                save_tasks(&tasks);
                println!("task {} completed", id);
            } else {
                println!("task not found");
            }
        }
        Some("delete") => {
            let id = match parse_id(&args) {
                Ok(id) => id,
                Err(message) => {
                    eprintln!("error: {message}");
                    return;
                }
            };

            let old_len = tasks.len();

            tasks.retain(|task| task.id != id);

            if tasks.len() < old_len {
                save_tasks(&tasks);
                println!("task {} deleted", id);
            } else {
                println!("task not found");
            }
        }
        Some("edit") => {
            let id = match parse_id(&args) {
                Ok(id) => id,
                Err(message) => {
                    eprintln!("error: {message}");
                    return;
                }
            };

            let title = args
                .get(3)
                .expect("please enter new task title")
                .to_string();

            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.title = title;
                save_tasks(&tasks);
                println!("task {} updated", id);
            } else {
                println!("task not found");
            }
        }
        _ => {
            println!("how to use: ");
            println!("  todo_cli add \"contents\"");
            println!("  todo_cli list");
            println!("  todo_cli delete <id>");
            println!("  todo_cli edit <id> \"new title\"");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_can_round_trip_json() {
        let original = Task {
            id: 1,
            title: "learn Rust".to_string(),
            completed: false,
        };

        let json = serde_json::to_string(&original).expect("failed to serialize task");

        let restored: Task = serde_json::from_str(&json).expect("failed to deserialize task");

        assert_eq!(original, restored);
    }
}
