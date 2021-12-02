use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut forward = 0;
    let mut depth = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (command, number) = line.split_once(' ').unwrap();
        let number: u32 = number.parse().unwrap();

        match command {
            "forward" => forward += number,
            "down" => depth += number,
            "up" => depth -= number,
            _ => panic!("unknown command"),
        }
    }

    println!("{}", forward * depth);
}
