use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

struct Grid {
    map: HashMap<(isize, isize), isize>,
    santa_turn: bool,
    santa_loc: (isize, isize),
    robo_loc: (isize, isize),
}

impl Grid {
    fn new() -> Grid {
        let mut map = HashMap::new();
        map.insert((0, 0), 1);
        Grid {
            map: map,
            santa_turn: true,
            santa_loc: (0, 0),
            robo_loc: (0, 0),
        }
    }

    fn visit_house(&mut self) {
        let coordinates = if self.santa_turn { self.santa_loc } else { self.robo_loc };
        *self.map
            .entry(coordinates)
            .or_insert(0) += 1;
    }

    fn num_visited(&self) -> usize {
        self.map.len()
    }

    fn move_santas(&mut self, direction: char) {
        if self.santa_turn {
            self.santa_loc = Self::next_coordinates(direction, self.santa_loc);
        } else {
            self.robo_loc = Self::next_coordinates(direction, self.robo_loc);
        }
        self.visit_house();
        self.santa_turn = !self.santa_turn;
    }

    fn next_coordinates(direction: char, coordinates: (isize, isize)) -> (isize, isize) {
        let (x, y) = coordinates;
        match direction {
            '^' => (x, y + 1),
            '>' => (x + 1, y),
            'v' => (x, y - 1),
            '<' => (x - 1, y),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("./input.txt").unwrap();
    file.read_to_string(&mut input);
    let mut grid = Grid::new();
    for direction in input.chars() {
        grid.move_santas(direction)
    }
    println!("visited: {}", grid.num_visited())
}
