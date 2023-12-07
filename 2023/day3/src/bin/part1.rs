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
            //print!("{:?}", c);
        }
        //println!();
    }

    println!("{:?}", symbols);
    println!("{:?}", nums);
    let mut prev_col: u32 = 0;
    let mut prev_row: u32 = 0;
    let mut prev_val = String::from("");
    let mut to_use: bool = false;
    let mut result: u32 = 0;
    for ((row, col), num) in nums{
        for (r, c) in &symbols{
            let r_diff = if &row < r{
                r - row
            }else{
                row - r
            };
            let c_diff = if &col < c{
                c - col
            }else{
                col - c
            };
            // dbg!(r_diff);
            // dbg!(r);
            // dbg!(row);
            // dbg!(c_diff);
            // dbg!(c);
            // dbg!(col);
            // println!();
            if r_diff <= 1 && c_diff <= 1{
                to_use = true;
                println!("{:?}", &to_use);
                break;
            }
        }
        if ((&prev_col + 1) == col) && (prev_row == row){
            //println!("row {}, col {}", row,col);
            prev_val.push(num);
        }
        else{ 
            if to_use{
                println!("{}", &prev_val);
                to_use = false;
                result += prev_val.parse::<u32>().unwrap();
            }

            prev_val = String::from(num.to_string());
        }

        prev_col = col;
        prev_row = row;

    }
    if to_use{
        println!("{}", &prev_val);
        result += prev_val.parse::<u32>().unwrap();
    }
    println!("{:?}", result);

}
