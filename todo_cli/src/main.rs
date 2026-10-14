use std::env;
use todo_cli::run;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = env::args().collect();

    match run(&args) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            std::process::ExitCode::FAILURE
        }
    }
}
