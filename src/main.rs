use std::env;
use std::fs;
mod rcp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args.len() < &2 {
        println!("You have to input a file");
        std::process::exit(1);
    }

    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    println!("{}", rcp::run(content));
}
