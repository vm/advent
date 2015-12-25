use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

struct Grid {
    map: HashMap<(usize, usize), usize>
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map: HashMap::new()
        }
    }

    fn visit_house(&mut self, x: usize, y: usize) {
        *self.map.entry((x, y)).or_insert(0) += 1;
    }
}

fn main() {
    let mut input = String::new();
    match File::open("./input.txt") {
        Ok(mut file) => file.read_to_string(&mut input),
        Err(why) => panic!("Exited because: {}", why),
    };
}
