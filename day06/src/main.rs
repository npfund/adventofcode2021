use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut fishies =
        lines[0]
            .split(',')
            .map(|f| f.parse::<i8>().unwrap())
            .fold(HashMap::new(), |mut acc, f| {
                let state = acc.entry(f).or_insert(0);
                *state += 1;

                acc
            });

    for _ in 0..256 {
        let mut to_add = 0;
        for state in 0..=8 {
            if state == 0 {
                to_add = *fishies.entry(0).or_insert(0);
            } else {
                let upper = *fishies.entry(state).or_insert(0);
                let entry = fishies.entry(state - 1).or_insert(0);
                *entry = upper;
            }
        }

        let sixes = fishies.entry(6).or_insert(0);
        *sixes += to_add;

        let eights = fishies.entry(8).or_insert(0);
        *eights = to_add;
    }

    println!("{}", fishies.values().sum::<u64>());
}
