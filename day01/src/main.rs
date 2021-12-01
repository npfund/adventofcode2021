use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let pairs = numbers.iter().zip(numbers.iter().skip(1));
    let increases = pairs.fold(
        0,
        |acc, (left, right)| {
            if left < right {
                acc + 1
            } else {
                acc
            }
        },
    );

    println!("increases: {}", increases);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut increases = 0;
    for index in 0..numbers.len() - 3 {
        let window1 = numbers[index] + numbers[index + 1] + numbers[index + 2];
        let window2 = numbers[index + 1] + numbers[index + 2] + numbers[index + 3];
        if window1 < window2 {
            increases += 1;
        }
    }

    println!("increases: {}", increases);
}
