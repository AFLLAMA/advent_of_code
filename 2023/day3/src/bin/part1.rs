use std::env;
use std::fs;
use std::ops::Range;

fn main() {
    let path: &str = &env::args().collect::<Vec<String>>()[1];
    let content = fs::read_to_string(path);
    let binding = content.expect("An input split into lines");
    let lines: Vec<&str> = binding.split("\n").collect::<Vec<&str>>();

    for line in lines{
        for i in 0..line.len(){
            print!("{:?}", line.get(i));
        }
        println!();
    }
}
