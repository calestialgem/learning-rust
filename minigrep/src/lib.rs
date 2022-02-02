use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn three_arguments() {
        let args = [
            String::from("running from test"),
            String::from("Hello"),
            String::from("World"),
        ];
        let config = Config::new(&args).expect("Could not create config?");
        assert_eq!(args[1], config.query);
        assert_eq!(args[2], config.filename);
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn less_than_three_arguments() {
        let args = [String::from("Hello")];
        Config::new(&args).unwrap();
    }
}
