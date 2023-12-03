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

    let strategy = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    let responce = HashMap::from([
        ("X", HashMap::from([
                  ("A", "Z"),
                  ("B", "X"),
                  ("C", "Y")])
        ),   
        ("Y", HashMap::from([
                  ("A", "X"),
                  ("B", "Y"),
                  ("C", "Z")])
        ),
        ("Z", HashMap::from([
                  ("A", "Y"),
                  ("B", "Z"),
                  ("C", "X")])
        ),
    ]);

    let mut score = 0;
    for line in lines{
        let plays: Vec<&str> = line.split_whitespace().collect();
        if plays.len() < 2{
            continue;
        }
        let (other, me) = (plays[0], plays[1]);
        let new_me = responce.get(&me).unwrap().get(&other).unwrap();
        score += strategy.get(me).unwrap();
        score += costs.get(new_me).unwrap();
    }
    println!("{}", score);
}
