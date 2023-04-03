use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_err| {
        println!("Problem parsing error");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}
