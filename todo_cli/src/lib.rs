use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}
#[derive(Debug, PartialEq)]
enum Command {
    Add(String),
    List,
    Done(usize),
    Delete(usize),
    Edit(usize, String),
    Help,
}
#[derive(Debug, PartialEq)]
pub enum CliError {
    Io(String),
    Json(String),
    MissingId,
    InvalidId,
    EmptyTitle,
    TaskNotFound(usize),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "file error: {err}"),
            CliError::Json(err) => write!(f, "json error: {err}"),
            CliError::MissingId => write!(f, "please enter task id"),
            CliError::InvalidId => write!(f, "task id must be a number"),
            CliError::EmptyTitle => write!(f, "please enter task contents"),
            CliError::TaskNotFound(id) => write!(f, "task {id} not found"),
        }
    }
}
impl std::error::Error for CliError {}
fn load_tasks() -> Result<Vec<Task>, CliError> {
    match fs::read_to_string("tasks.json") {
        Ok(contents) => serde_json::from_str(&contents)
            .map_err(|error| CliError::Json(format!("failed to parse tasks.json: {error}"))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(CliError::Io(format!("failed to read tasks.json: {error}"))),
    }
}
fn save_tasks(tasks: &[Task]) -> Result<(), CliError> {
    let contents = serde_json::to_string_pretty(tasks)
        .map_err(|error| CliError::Json(format!("failed to serialize tasks: {error}")))?;

    fs::write("tasks.json", contents)
        .map_err(|error| CliError::Io(format!("failed to write tasks.json: {error}")))?;

    Ok(())
}
fn parse_id(args: &[String]) -> Result<usize, CliError> {
    let raw_id = args.get(2).ok_or(CliError::MissingId)?;

    raw_id.parse::<usize>().map_err(|_| CliError::InvalidId)
}
fn parse_command(args: &[String]) -> Result<Command, CliError> {
    let action = match args.get(1) {
        Some(action) => action.as_str(),
        None => return Ok(Command::Help),
    };
    match action {
        "add" => {
            let title = match args.get(2) {
                Some(title) if !title.trim().is_empty() => title.to_string(),
                _ => return Err(CliError::EmptyTitle),
            };
            Ok(Command::Add(title))
        }
        "list" => Ok(Command::List),

        "done" => {
            let id = parse_id(args)?;
            Ok(Command::Done(id))
        }

        "delete" => {
            let id = parse_id(args)?;
            Ok(Command::Delete(id))
        }

        "edit" => {
            let id = parse_id(args)?;
            let title = match args.get(3) {
                Some(title) if !title.trim().is_empty() => title.to_string(),
                _ => return Err(CliError::EmptyTitle),
            };
            Ok(Command::Edit(id, title))
        }
        _ => Ok(Command::Help),
    }
}
fn complete_task(tasks: &mut [Task], id: usize) -> Result<(), CliError> {
    let task = tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or(CliError::TaskNotFound(id))?;

    task.completed = true;
    Ok(())
}
fn edit_task(tasks: &mut [Task], id: usize, title: String) -> Result<(), CliError> {
    let task = tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or(CliError::TaskNotFound(id))?;

    task.title = title;
    Ok(())
}
pub fn run(args: &[String]) -> Result<(), CliError> {
    let command = parse_command(args)?;
    let mut tasks = load_tasks()?;

    match command {
        Command::Add(title) => {
            let new_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

            tasks.push(Task {
                id: new_id,
                title,
                completed: false,
            });
            save_tasks(&tasks)?;
            println!("add task: {new_id}");
        }
        Command::List => {
            for task in &tasks {
                let status = if task.completed { "x" } else { " " };
                println!("{} [{}] {}", task.id, status, task.title);
            }
        }
        Command::Done(id) => {
            complete_task(&mut tasks, id)?;
            save_tasks(&tasks)?;
            println!("task {id} completed");
        }
        Command::Delete(id) => {
            let old_len = tasks.len();
            tasks.retain(|task| task.id != id);

            if tasks.len() < old_len {
                save_tasks(&tasks)?;
                println!("task {id} deleted");
            } else {
                return Err(CliError::TaskNotFound(id));
            }
        }
        Command::Edit(id, title) => {
            edit_task(&mut tasks, id, title)?;
            save_tasks(&tasks)?;
            println!("task {id} updated");
        }
        Command::Help => {
            println!("how to use:");
            println!("  todo_cli add \"contents\"");
            println!("  todo_cli list");
            println!("  todo_cli done <id>");
            println!("  todo_cli delete <id>");
            println!("  todo_cli edit <id> \"new title\"");
        }
    }

    Ok(())
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
    #[test]
    fn parse_id_accepts_valid_number() {
        let args = vec!["todo_cli".to_string(), "done".to_string(), "42".to_string()];

        assert_eq!(parse_id(&args), Ok(42));
    }

    #[test]
    fn parse_id_rejects_missing_id() {
        let args = vec!["todo_cli".to_string(), "done".to_string()];

        assert_eq!(parse_id(&args), Err(CliError::MissingId));
    }
    #[test]
    fn parse_id_rejects_invalid_number() {
        let args = vec![
            "todo_cli".to_string(),
            "done".to_string(),
            "abc".to_string(),
        ];

        assert_eq!(parse_id(&args), Err(CliError::InvalidId));
    }
    #[test]
    fn parse_command_parses_add() {
        let args = vec![
            "todo_cli".to_string(),
            "add".to_string(),
            "buy milk".to_string(),
        ];

        assert_eq!(
            parse_command(&args),
            Ok(Command::Add("buy milk".to_string()))
        );
    }
    #[test]
    fn parse_command_parses_list() {
        let args = vec!["todo_cli".to_string(), "list".to_string()];

        assert_eq!(parse_command(&args), Ok(Command::List));
    }
    #[test]
    fn parse_command_parses_done() {
        let args = vec!["todo_cli".to_string(), "done".to_string(), "1".to_string()];

        assert_eq!(parse_command(&args), Ok(Command::Done(1)));
    }

    #[test]
    fn parse_command_parses_delete() {
        let args = vec![
            "todo_cli".to_string(),
            "delete".to_string(),
            "2".to_string(),
        ];

        assert_eq!(parse_command(&args), Ok(Command::Delete(2)));
    }

    #[test]
    fn parse_command_parses_edit() {
        let args = vec![
            "todo_cli".to_string(),
            "edit".to_string(),
            "1".to_string(),
            "new title".to_string(),
        ];

        assert_eq!(
            parse_command(&args),
            Ok(Command::Edit(1, "new title".to_string()))
        );
    }
    #[test]
    fn parse_command_returns_help_for_missing_or_unknown() {
        let no_args = vec!["todo_cli".to_string()];
        assert_eq!(parse_command(&no_args), Ok(Command::Help));

        let unknown = vec!["todo_cli".to_string(), "foo".to_string()];
        assert_eq!(parse_command(&unknown), Ok(Command::Help));
    }
    #[test]
    fn parse_command_rejects_empty_add_title() {
        let no_title = vec!["todo_cli".to_string(), "add".to_string()];
        assert_eq!(parse_command(&no_title), Err(CliError::EmptyTitle));

        let whitespace_title = vec!["todo_cli".to_string(), "add".to_string(), "   ".to_string()];
        assert_eq!(parse_command(&whitespace_title), Err(CliError::EmptyTitle));
    }

    #[test]
    fn parse_command_rejects_missing_edit_title() {
        let args = vec!["todo_cli".to_string(), "edit".to_string(), "1".to_string()];
        assert_eq!(parse_command(&args), Err(CliError::EmptyTitle));
    }
    #[test]
    fn parse_id_rejects_negative_number() {
        let args = vec!["todo_cli".to_string(), "done".to_string(), "-1".to_string()];

        assert_eq!(parse_id(&args), Err(CliError::InvalidId));
    }
    #[test]
    fn parse_id_rejects_number_above_usize_max() {
        let too_large = format!("{}0", usize::MAX);
        let args = vec!["todo_cli".to_string(), "done".to_string(), too_large];

        assert_eq!(parse_id(&args), Err(CliError::InvalidId));
    }
    #[test]
    fn complete_task_marks_only_target_as_completed() {
        let mut tasks = vec![
            Task {
                id: 1,
                title: "learn Rust".to_string(),
                completed: false,
            },
            Task {
                id: 2,
                title: "write tests".to_string(),
                completed: false,
            },
        ];

        assert_eq!(complete_task(&mut tasks, 2), Ok(()));
        assert!(!tasks[0].completed);
        assert!(tasks[1].completed);
    }
    #[test]
    fn complete_task_preserves_tasks_when_id_is_missing() {
        let original = Task {
            id: 1,
            title: "learn Rust".to_string(),
            completed: false,
        };
        let mut tasks = vec![Task {
            id: original.id,
            title: original.title.clone(),
            completed: original.completed,
        }];

        assert_eq!(
            complete_task(&mut tasks, 99),
            Err(CliError::TaskNotFound(99))
        );
        assert_eq!(tasks, vec![original]);
    }
    #[test]
    fn edit_task_changes_title_without_changing_id_or_status() {
        let mut tasks = vec![Task {
            id: 7,
            title: "learn Rust".to_string(),
            completed: true,
        }];

        assert_eq!(
            edit_task(&mut tasks, 7, "practice Rust".to_string()),
            Ok(())
        );

        assert_eq!(
            tasks,
            vec![Task {
                id: 7,
                title: "practice Rust".to_string(),
                completed: true,
            }]
        );
    }
    #[test]
    fn edit_task_preserves_tasks_when_id_is_missing() {
        let mut tasks = vec![Task {
            id: 7,
            title: "learn Rust".to_string(),
            completed: true,
        }];

        assert_eq!(
            edit_task(&mut tasks, 99, "practice Rust".to_string()),
            Err(CliError::TaskNotFound(99))
        );

        assert_eq!(
            tasks,
            vec![Task {
                id: 7,
                title: "learn Rust".to_string(),
                completed: true,
            }]
        );
    }
}
