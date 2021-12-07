use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let positions = lines[0]
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut min = i64::MAX;

    for end in &positions {
        let mut fuel = 0;
        for start in &positions {
            fuel += (*start - *end).abs();
        }

        if fuel < min {
            min = fuel;
        }
    }

    println!("{}", min);
}
