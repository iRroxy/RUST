#![allow(dead_code)]
use std::env;
use std::error::Error;
use std::fs;

fn search_by<'a, F>(contents: &'a str, mut predicate: F) -> Vec<(usize, &'a str)>
where
    F: FnMut(&str) -> bool,
{
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| predicate(line))
        .map(|(index, line)| (index + 1, line))
        .collect()
}

pub fn search_with_options<'a>(
    query: &str,
    contents: &'a str,
    ignore_case: bool,
) -> Vec<(usize, &'a str)> {
    if ignore_case {
        let query_lower = query.to_lowercase();
        search_by(contents, |line| line.to_lowercase().contains(&query_lower))
    } else {
        search_by(contents, |line| line.contains(query))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_with_options(query, contents, false)
        .into_iter()
        .map(|(_, line)| line)
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_with_options(query, contents, true)
        .into_iter()
        .map(|(_, line)| line)
        .collect()
}

pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    search_with_options(query, contents, false)
}

pub fn search_case_insensitive_with_line_numbers<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    search_with_options(query, contents, true)
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
}

fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> Result<(String, String, bool, bool), String> {
    args.next();

    let mut query = None;
    let mut file_path = None;
    let mut show_line_numbers = false;
    let mut ignore_case = env::var("IGNORE_CASE").is_ok();

    for arg in args {
        match arg.as_str() {
            "-n" => show_line_numbers = true,
            "-i" => ignore_case = true,
            _ if query.is_none() => query = Some(arg),
            _ if file_path.is_none() => file_path = Some(arg),
            _ => return Err(format!("Unknown argument: {}", arg)),
        }
    }

    let query = query.ok_or_else(|| "Didn't get a query string".to_string())?;
    let file_path = file_path.ok_or_else(|| "Didn't get a file path".to_string())?;

    Ok((query, file_path, ignore_case, show_line_numbers))
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, String> {
        let (query, file_path, ignore_case, show_line_numbers) = parse_args(args)?;

        Ok(Config {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
        })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let results = search_with_options(&config.query, &contents, config.ignore_case);

        for (index, line) in results {
            if config.show_line_numbers {
                println!("{index}: {line}");
            } else {
                println!("{line}");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result() {
        assert_eq!(
            Vec::<&str>::new(),
            search("xyz", "Rust:\nsafe, fast, productive.")
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn line_numbers() {
        let contents = "Hello\nI love Rust\nGoodbye\nRust is fast";

        assert_eq!(
            vec![(2, "I love Rust"), (4, "Rust is fast")],
            search_with_line_numbers("Rust", contents)
        );
    }

    #[test]
    fn case_insensitive_line_numbers() {
        let contents = "Hello\nI love rust\nGoodbye\nRust is fast";

        assert_eq!(
            vec![(2, "I love rust"), (4, "Rust is fast")],
            search_case_insensitive_with_line_numbers("RUST", contents)
        );
    }

    #[test]
    fn missing_query() {
        let args = vec!["minigrep".to_string()];
        let result = Config::build(args.into_iter());

        assert_eq!(result.unwrap_err(), "Didn't get a query string");
    }

    #[test]
    fn missing_file_path() {
        let args = vec!["minigrep".to_string(), "Rust".to_string()];
        let result = Config::build(args.into_iter());

        assert_eq!(result.unwrap_err(), "Didn't get a file path");
    }

    #[test]
    fn unknown_argument() {
        let args = vec![
            "minigrep".to_string(),
            "Rust".to_string(),
            "poem.txt".to_string(),
            "-x".to_string(),
        ];

        let result = Config::build(args.into_iter());

        assert_eq!(result.unwrap_err(), "Unknown argument: -x");
    }

    #[test]
    fn ignore_case_flag() {
        let args = vec![
            "minigrep".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
            "-i".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();

        assert!(config.ignore_case);
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_path, "poem.txt");
    }

    #[test]
    fn flag_order_independent() {
        let args1 = vec![
            "minigrep".to_string(),
            "-n".to_string(),
            "Rust".to_string(),
            "poem.txt".to_string(),
        ];
        let config1 = Config::build(args1.into_iter()).unwrap();
        assert_eq!(
            config1,
            Config {
                query: "Rust".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: env::var("IGNORE_CASE").is_ok(),
                show_line_numbers: true,
            }
        );

        let args2 = vec![
            "minigrep".to_string(),
            "Rust".to_string(),
            "poem.txt".to_string(),
            "-n".to_string(),
        ];
        let config2 = Config::build(args2.into_iter()).unwrap();
        assert_eq!(
            config2,
            Config {
                query: "Rust".to_string(),
                file_path: "poem.txt".to_string(),
                ignore_case: env::var("IGNORE_CASE").is_ok(),
                show_line_numbers: true,
            }
        );
    }

    #[test]
    fn test_search_with_options() {
        let contents = "Rust\nrust\nFast";
        assert_eq!(
            search_with_options("rust", contents, false),
            vec![(2, "rust")]
        );
        assert_eq!(
            search_with_options("rust", contents, true),
            vec![(1, "Rust"), (2, "rust")]
        );
    }
}
