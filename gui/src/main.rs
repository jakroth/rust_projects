#![warn(clippy::cargo)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::env;
use std::process;
use gui::Draw;

#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox...");
        // code to actually draw a button
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    process::exit(1);
}


