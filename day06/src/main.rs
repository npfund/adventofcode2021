use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut fishies = lines[0]
        .split(',')
        .map(|f| f.parse().unwrap())
        .collect::<Vec<i8>>();

    for _ in 0..80 {
        let mut to_add = 0;
        for fish in fishies.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                to_add += 1;
            } else {
                *fish -= 1;
            }
        }

        fishies.extend(vec![8; to_add].iter());
    }

    println!("{}", fishies.len());
}
