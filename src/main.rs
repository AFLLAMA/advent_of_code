use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let mut max_sum: u32 = 0;
    let mut curr: u32 = 0;
    if let Ok(lines) = read_lines(file_path){
        for line in lines {
            if let Ok(parsed_num) = line.expect("number").parse::<u32>() {
                curr = &curr + parsed_num;
                if curr > max_sum{
                    max_sum = curr
                }
            }
            else {
                curr = 0;
            }
        }
    }
    println!("Max sum is : {}", max_sum);

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

