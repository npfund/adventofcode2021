use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut points = HashMap::new();
    for line in file.lines().map(|l| l.unwrap()) {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(',').unwrap();
        let (x2, y2) = end.split_once(',').unwrap();

        if x1 == x2 {
            let x: u32 = x1.parse().unwrap();
            let y1: u32 = y1.parse().unwrap();
            let y2: u32 = y2.parse().unwrap();

            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            for y in y1..=y2 {
                let point = points.entry((x, y)).or_insert(0);
                *point += 1;
            }
        }

        if y1 == y2 {
            let x1: u32 = x1.parse().unwrap();
            let x2: u32 = x2.parse().unwrap();
            let y: u32 = y1.parse().unwrap();

            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

            for x in x1..=x2 {
                let point = points.entry((x, y)).or_insert(0);
                *point += 1;
            }
        }
    }

    let count = points.values().filter(|p| **p > 1).count();

    println!("{}", count);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut points = HashMap::new();
    for line in file.lines().map(|l| l.unwrap()) {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(',').unwrap();
        let (x2, y2) = end.split_once(',').unwrap();

        let x1: i32 = x1.parse().unwrap();
        let x2: i32 = x2.parse().unwrap();
        let y1: i32 = y1.parse().unwrap();
        let y2: i32 = y2.parse().unwrap();

        let mut step = (0, 0);
        if x1 < x2 {
            step.0 = 1;
        } else if x1 > x2 {
            step.0 = -1;
        } else {
            step.0 = 0;
        }

        if y1 < y2 {
            step.1 = 1;
        } else if y1 > y2 {
            step.1 = -1;
        } else {
            step.1 = 0;
        }

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            let point = points.entry((x, y)).or_insert(0);
            *point += 1;

            x += step.0;
            y += step.1;
        }

        let point = points.entry((x, y)).or_insert(0);
        *point += 1;
    }

    let count = points.values().filter(|p| **p > 1).count();

    println!("{}", count);
}
