use std::env;
use std::process;
use minigrep::Config;

fn main(){
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config){
        println!("Applicaton Error: {}", e);
        process::exit(1);
    };
}


