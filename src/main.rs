use std::{env, process};

use find_replace::{Config, run};

fn main() {
    let cfg = Config::new(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        process::exit(1);
    });

    match run(cfg) {
        Ok(text) => println!("{}", text),
        Err(err) => {
            eprintln!("Error while parsing: {}", err);
            process::exit(1);
        }
    }
}