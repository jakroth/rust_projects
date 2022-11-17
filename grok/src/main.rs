#![warn(clippy::cargo)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::env;
use std::process;
use grok::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grok::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

