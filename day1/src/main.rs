use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut input = String::new();
    match File::open("./input.txt") {
        Ok(mut file) => file.read_to_string(&mut input),
        Err(why) => panic!("Exited because: {}", why),
    };
    let mut total = 0;
    let mut first_neg = None;
    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => total += 1,
            ')' => total -= 1,
            _ => unreachable!("Shouldn't get any other chars"),
        };
        if let None = first_neg {
            if total < 0 {
                first_neg = Some(i + 1);
            }
        }
    }
    println!("Total: {}", total);
    println!("Position {} was the first to lead to a negative total.", first_neg.unwrap());
}
