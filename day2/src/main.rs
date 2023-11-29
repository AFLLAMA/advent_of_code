use std::env;
use std::fs;

fn main() {
    let path: &str = &env::args().collect::<Vec<String>>()[2];
    let content = fs::read_to_string(path);
    let binding = content.expect("reason");
    let lines: Vec<&str> = binding.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", lines);
    //println!("Usage: {} <file-path>", env::args().next().unwrap_or_else(|| String::from("program")));
}
