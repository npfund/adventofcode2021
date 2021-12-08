use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let count = file.lines().map(|l| l.unwrap()).fold(0, |acc, line| {
        let (_, output) = line.split_once(" | ").unwrap();
        let matching = output
            .split(' ')
            .filter(|p| matches!(p.len(), 2 | 3 | 4 | 7))
            .count();

        acc + matching
    });

    println!("{}", count);
}
