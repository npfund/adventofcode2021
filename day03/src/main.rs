use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines: Vec<Vec<u32>> = file
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let mut selected = lines.clone();
    for i in 0..12 {
        let most = count_bits(&selected, i);

        selected = selected.into_iter().filter(|l| l[i] == most).collect();
        if selected.len() == 1 {
            break;
        }
    }

    let oxy = to_int(&selected[0]);

    let mut selected = lines.clone();
    for i in 0..12 {
        let least = if count_bits(&selected, i) == 0 { 1 } else { 0 };

        selected = selected.into_iter().filter(|l| l[i] == least).collect();
        if selected.len() == 1 {
            break;
        }
    }

    let co2 = to_int(&selected[0]);

    println!("{}", oxy * co2);
}

fn count_bits(lines: &Vec<Vec<u32>>, position: usize) -> u32 {
    let (zero, one) = lines.iter().fold((0, 0), |mut acc, l| {
        if l[position] == 0 {
            acc.0 += 1;
        } else {
            acc.1 += 1;
        }

        acc
    });

    if zero > one {
        0
    } else {
        1
    }
}
