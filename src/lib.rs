use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len( ) < 3 {
            //panic!("not enough arguments!");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
//        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);

    Ok(())

}