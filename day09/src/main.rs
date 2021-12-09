use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut map = HashMap::new();
    for (y, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        for (x, height) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((x as i32, y as i32), height);
        }
    }

    let mut sum = 0;
    for ((x, y), &height) in &map {
        let up = *map.get(&(*x, y - 1)).unwrap_or(&u32::MAX);
        let right = *map.get(&(x + 1, *y)).unwrap_or(&u32::MAX);
        let down = *map.get(&(*x, y + 1)).unwrap_or(&u32::MAX);
        let left = *map.get(&(x - 1, *y)).unwrap_or(&u32::MAX);

        if height < up && height < right && height < down && height < left {
            sum += 1 + height;
        }
    }

    println!("{}", sum);
}
