use std::env;
use std::fs;


fn main() {
    let path: &str = &env::args().collect::<Vec<String>>()[1];
    let content = fs::read_to_string(path);
    let binding = content.expect("An input split into lines");
    let lines: Vec<&str> = binding.split("\n").collect::<Vec<&str>>();

    let mut symbols: Vec<(u32,u32)> = Vec::new();
    let mut nums: Vec<((u32,u32),char)> = Vec::new();
    for i in 0..lines.len(){
        for (j,c) in lines[i].char_indices(){  
            if !c.is_digit(10){
                if c == '.'{
                    continue;
                }
                else{
                    symbols.push((i.try_into().unwrap(),j.try_into().unwrap()));
                }
            }
            else{
                nums.push(((i.try_into().unwrap(),j.try_into().unwrap()),c));
            }
            print!("{:?}", c);
        }
        println!();
    }

    println!("{:?}", symbols);
    println!("{:?}", nums);
    // let mut new_nums : Vec
}
