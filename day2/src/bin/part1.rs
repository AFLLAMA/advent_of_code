use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let path: &str = &env::args().collect::<Vec<String>>()[1];
    let content = fs::read_to_string(path);
    let binding = content.expect("An input split into lines");
    let lines: Vec<&str> = binding.split("\n").collect::<Vec<&str>>();

    let costs = HashMap::from([
        ("A", 1),
        ("X", 1),
        ("B", 2),
        ("Y", 2),
        ("C", 3),
        ("Z", 3),
    ]);
    let mut score = 0;
    for line in lines{
        let plays: Vec<&str> = line.split_whitespace().collect();
        if plays.len() < 2{
            continue;
        }
        let (other, me) = (plays[0], plays[1]);
        let win_score = match (other, me) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 3,
        };
        score += win_score;
        score += costs.get(&me).unwrap();
    }
    println!("{}", score);
}
