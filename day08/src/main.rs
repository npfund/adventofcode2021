use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let sum = file.lines().map(|l| l.unwrap()).fold(0, |acc, line| {
        let (input, output) = line.split_once(" | ").unwrap();
        let mut nums = vec![String::new(); 10];

        let patterns = input.split(' ').collect::<Vec<_>>();
        for pattern in &patterns {
            if pattern.len() == 2 {
                nums[1] = sort(pattern);
            }

            if pattern.len() == 3 {
                nums[7] = sort(pattern);
            }

            if pattern.len() == 4 {
                nums[4] = sort(pattern);
            }

            if pattern.len() == 7 {
                nums[8] = sort(pattern);
            }
        }

        // if len is 6 and does not contain 1 => 6
        // else if contains 4 => 9
        // else => 0

        // if len is 5 and contains 1 => 3
        // else if diff 4 == 1 => 5
        // else => 2
        for pattern in &patterns {
            if pattern.len() == 6 {
                let sorted = sort(pattern);
                if diff(&nums[1], &sorted) != 0 {
                    nums[6] = sorted;
                } else if diff(&nums[4], &sorted) == 0 {
                    nums[9] = sorted;
                } else {
                    nums[0] = sorted;
                }
            }

            if pattern.len() == 5 {
                let sorted = sort(pattern);
                if diff(&nums[1], &sorted) == 0 {
                    nums[3] = sorted;
                } else if diff(&nums[4], &sorted) == 1 {
                    nums[5] = sorted;
                } else {
                    nums[2] = sorted;
                }
            }
        }

        let mut num = 0;
        let patterns = output.split(' ').collect::<Vec<_>>();
        for pattern in &patterns {
            for i in 0..nums.len() {
                if sort(pattern) == nums[i] {
                    num = (num * 10) + i as i32;
                }
            }
        }

        acc + num
    });

    println!("{}", sum);
}

fn sort(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| b.cmp(a));

    chars.into_iter().rev().collect()
}

fn diff(a: &str, b: &str) -> u32 {
    let mut diff = 0;
    for c in a.chars() {
        if !b.contains(c) {
            diff += 1;
        }
    }

    diff
}
