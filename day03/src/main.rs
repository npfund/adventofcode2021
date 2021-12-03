use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines: Vec<Vec<u32>> = file
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let total = lines.len() as u32;

    let summed: Vec<u32> =
        lines
            .iter()
            .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |mut acc, l| {
                for i in 0..l.len() {
                    acc[i] += l[i]
                }

                acc
            });

    let gamma = summed
        .iter()
        .map(|b| if *b > total / 2 { 1 } else { 0 })
        .collect::<Vec<u32>>();
    let gamma = to_int(&gamma);

    let epsilon = summed
        .iter()
        .map(|b| if *b > total / 2 { 0 } else { 1 })
        .collect::<Vec<u32>>();
    let epsilon = to_int(&epsilon);

    println!("{}", gamma * epsilon);
}

fn to_int(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}
