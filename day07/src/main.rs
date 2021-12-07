use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let positions = lines[0]
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect::<Vec<i64>>();

    let min_position = positions.iter().min().unwrap();
    let max_position = positions.iter().max().unwrap();
    let mut min = i64::MAX;

    for end in *min_position..*max_position {
        let mut fuel = 0;
        for start in &positions {
            let steps = (*start - end).abs();
            fuel += (steps * (1 + steps)) / 2;
        }

        if fuel < min {
            min = fuel;
        }
    }

    println!("{}", min);
}
