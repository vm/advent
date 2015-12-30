#![feature(iter_arith)]

use std::io::prelude::*;
use std::fs::File;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn new(line: String) -> Present {
        let (length, width, height) = Self::parse_line(line);
        Present {
            length: length,
            width: width,
            height: height,
        }
    }

    fn parse_line(line: String) -> (usize, usize, usize) {
        let split = line
            .trim()
            .split('x')
            .collect::<Vec<_>>();
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

    fn ribbon_len(&self) -> usize {
        let mut dimensions = vec![self.length, self.height, self.width];
        dimensions.sort();
        let (first, second) = (dimensions[0], dimensions[1]);
        first * 2 + second * 2 + dimensions.iter().fold(1, |acc, dim| dim * acc)
    }
}

struct Presents {
    presents: Vec<Present>,
}

impl Presents {
    fn new() -> Presents {
        Presents {
            presents: Vec::new()
        }
    }

    fn add(&mut self, present: Present) {
        self.presents.push(present)
    }

    fn wrapping_paper(&self) -> usize {
        self.presents
            .iter()
            .map(Present::surface_area)
            .sum()
    }

    fn ribbon_len(&self) -> usize {
        self.presents
            .iter()
            .map(Present::ribbon_len)
            .sum()
    }
}


fn main() {
    let mut input = String::new();
    let mut file = File::open("./input.txt").unwrap();
    file.read_to_string(&mut input);
    let mut presents = Presents::new();
    for line in input.lines() {
        presents.add(Present::new(line.to_owned()))
    }
    println!("total sq feet of wrapping paper: {}", presents.wrapping_paper());
    println!("total feet of ribbon: {}", presents.ribbon_len());
}
