use std::error::Error;
use std::fs;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents){
        println!("{:?}",line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
        //(query, filename)
    }
}
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
let mut results=Vec::new();
for line in contents.lines(){
    if line.contains(query){
results.push(line);
    }
    
}
results
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn one_result(){
        let query="wasif";
        let content="\
        My name is wasif shah
        Welcome to my profile
        I am a Rust developer";
assert_eq!(vec!["My name is wasif shah"],search(query,content));
    }
}