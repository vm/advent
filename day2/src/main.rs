use std::io::prelude::*;
use std::fs::File;

struct Dimensions {
    length: usize,
    width: usize,
    height: usize,
}

impl Dimensions {
    fn new(line: String) -> Dimensions {
        let (length, width, height) = Self::parse_line(line);
        Dimensions {
            length: length,
            width: width,
            height: height,
        }
    }

    fn parse_line(line: String) -> (usize, usize, usize) {
        let split = line.trim().split('x').collect::<Vec<_>>();
        (split[0].parse().unwrap(),
         split[1].parse().unwrap(),
         split[2].parse().unwrap())
    }

    fn surface_area(&self) -> usize {
        let mut sides = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        sides.sort();
        let smallest = sides[0];
        sides
            .iter()
            .fold(smallest, |acc, side| 2 * side + acc)
    }
}

struct Presents {
    presents: Vec<Dimensions>,
}

impl Presents {
    fn new() -> Presents {
        Presents {
            presents: Vec::new()
        }
    }

    fn add(&mut self, present: Dimensions) {
        self.presents.push(present)
    }

    fn wrapping_paper(&mut self) -> usize {
        self.presents
            .iter()
            .fold(0, |acc, present| present.surface_area() + acc)
    }
}


fn main() {
    let mut input = String::new();
    match File::open("./input.txt") {
        Ok(mut file) => file.read_to_string(&mut input),
        Err(why) => panic!("Exited because: {}", why),
    };
    let mut presents = Presents::new();
    for line in input.split('\n') {
        if line == "" {
            break
        }
        presents.add(Dimensions::new(line.to_owned()))
    }
    println!("total sq feet of wrapping paper: {}", presents.wrapping_paper())
}
