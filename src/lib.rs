use std::{env, error::Error, fs};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    program_call: String,
    pub query: String,
    pub target_filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let config = Self {
            program_call: args.next().unwrap(),
            query: match args.next() {
                Some(arg) => arg,
                None => return Err("Did not get a query string!".to_string()),
            },
            target_filename: match args.next() {
                Some(arg) => arg,
                None => return Err("Did not get a target file!".to_string()),
            },
            ignore_case: env::var("MINIGREP_IGNORE_CASE").is_ok(),
        };

        config.parse_flags(args)
    }

    fn parse_flags(mut self, flags: impl Iterator<Item = String>) -> Result<Self, String> {
        for flag in flags {
            match flag.as_str() {
                "-i" | "--ignore-case" => self.ignore_case = true,
                invalid_flag => return Err(format!("flag {} is invalid", invalid_flag)),
            };
        }
        Ok(self)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(config.target_filename)?;
    let results = if config.ignore_case {
        search_insensitive_case(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive_case(query, contents)
        )
    }
}
