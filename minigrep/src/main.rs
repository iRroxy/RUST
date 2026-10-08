use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("error! {err}");
        process::exit(1);
    });
    if let Err(e) = Config::run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
