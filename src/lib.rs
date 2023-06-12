use std::{error::Error, fs};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    program_call: String,
    pub query: String,
    pub target_filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(
                "Minigrep must be called with search term and filename arguments".to_owned(),
            );
        }
        Ok(Self {
            program_call: args[0].clone(),
            query: args[1].clone(),
            target_filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(config.target_filename)?;
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
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
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }
}
