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

#[cfg(test)]
mod tests {

    use super::*;

    const DUMMY_PARAM: &str = "a";

    #[test]
    fn config_pass_enough_params_succeed() {
        let test_params = vec![
            String::from(DUMMY_PARAM),
            String::from(DUMMY_PARAM),
            String::from(DUMMY_PARAM),
            String::from(DUMMY_PARAM),
        ];

        let actual = Config::new(test_params).unwrap();

        assert_eq!(String::from(DUMMY_PARAM), actual.file);
        assert_eq!(String::from(DUMMY_PARAM), actual.find_replace);
        assert_eq!(String::from(DUMMY_PARAM), actual.replace_for);
    }

    #[test]
    #[should_panic]
    fn config_pass_too_few_params_fail() {
        let test_params = vec![
            String::from(DUMMY_PARAM),
        ];

        let _actual = Config::new(test_params).unwrap();
    }

    const FIND_PATTERN: &str = "the";
    const REPLACE_PATTERN: &str = "xxx";
    const INPUT_LITERAL: &str = "This is the shit!";
    const MATCHED_OUTPUT_LITERAL: &str = "This is xxx shit!\n";

    #[test]
    fn find_and_replace_find_match_and_replace() {
        let find_pattern = String::from(FIND_PATTERN);
        let replace_pattern = String::from(REPLACE_PATTERN);

        let input = String::from(INPUT_LITERAL);
        let expected = String::from(MATCHED_OUTPUT_LITERAL);

        let actual = find_and_replace(input, find_pattern, replace_pattern);

        assert_eq!(expected, actual);
    }

    const UNMATCHED_OUTPUT_LITERAL: &str = "This is the shit!\n";

    #[test]
    fn find_and_replace_no_matches() {
        let find_pattern = String::from(REPLACE_PATTERN);
        let replace_pattern = String::from(FIND_PATTERN);

        let input = String::from(INPUT_LITERAL);
        let expected = String::from(UNMATCHED_OUTPUT_LITERAL);

        let actual = find_and_replace(input, find_pattern, replace_pattern);

        assert_eq!(expected, actual);
    }
}