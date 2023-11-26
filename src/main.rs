use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let mut max_sums = BinaryHeap::new();
    let mut curr: u32 = 0;
    if let Ok(lines) = read_lines(file_path){
        for line in lines {
            if let Ok(parsed_num) = line.expect("number").parse::<u32>() {
                curr = &curr + parsed_num;
            }
            else {
                max_sums.push(Reverse(curr));
                curr = 0;
            }
        }
    }
    if curr != 0{
        max_sums.push(Reverse(curr));
    }
    
    while max_sums.len() > 3{
       max_sums.pop();
    }
    println!("{:?}",&max_sums);
    println!("Max sum is : {}", max_sums.pop().unwrap().0 + max_sums.pop().unwrap().0 + max_sums.pop().unwrap().0);
    
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

