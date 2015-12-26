use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn to_direction(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => unreachable!(),
    }
}

type Coordinate = (isize, isize);

struct Grid {
    map: HashMap<Coordinate, isize>,
    santa_turn: bool,
    santa_loc: Coordinate,
    robo_loc: Coordinate,
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

    fn move_santas(&mut self, direction: Direction) {
        if self.santa_turn {
            self.santa_loc = Self::next_coordinates(direction, self.santa_loc);
        } else {
            self.robo_loc = Self::next_coordinates(direction, self.robo_loc);
        }
        self.visit_house();
        self.santa_turn = !self.santa_turn;
    }

    fn next_coordinates(direction: Direction, coordinates: Coordinate) -> Coordinate {
        let (x, y) = coordinates;
        match direction {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("./input.txt").unwrap();
    file.read_to_string(&mut input);
    let mut grid = Grid::new();
    for direction in input.chars().map(to_direction) {
        grid.move_santas(direction);
    }
    println!("visited: {}", grid.num_visited())
}
