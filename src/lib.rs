use std::{fs, error::Error};

#[derive(Debug)]
pub struct Config {
    file: String,
    find_replace: String,
    replace_for: String,
}

const NOT_ENOUGH_PARAMETERS: &str = "Not enough parameters!";

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err(NOT_ENOUGH_PARAMETERS)
        }

        Ok(Config {
            file: args[1].clone(),
            find_replace: args[2].clone(),
            replace_for: args[3].clone(),
        })
    }
}

fn find_and_replace(
    file_contents: String, find_replace: String, replace_for: String) -> String {
    let mut result: String = String::from("");

    for line in file_contents.lines() {
        let modified_line = line.replace(
            &find_replace[..], 
            &replace_for[..]
        ) + "\n";
        result.push_str(&modified_line[..]);
    }

    result
}

pub fn run(cfg: Config) -> Result<String, Box<dyn Error>> {
    let file_contents = fs::read_to_string(cfg.file)?;
    Ok(find_and_replace(file_contents, cfg.find_replace, cfg.replace_for))
}