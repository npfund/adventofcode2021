use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut map = HashMap::new();
    for (y, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        for (x, energy) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((x as i32, y as i32), energy);
        }
    }

    let mut flashes = 0;
    for _ in 0..100 {
        let mut flashing: HashSet<(i32, i32)> = HashSet::new();
        map.values_mut().for_each(|octopus| *octopus += 1);

        loop {
            let current_flash = flashing.len();
            for y in 0..10 {
                for x in 0..10 {
                    let octopus = map.entry((x, y)).or_default();
                    if *octopus > 9 && !flashing.contains(&(x, y)) {
                        flashing.insert((x, y));
                        for ystep in -1..=1 {
                            for xstep in -1..=1 {
                                map.entry((x + xstep, y + ystep)).and_modify(|a| *a += 1);
                            }
                        }
                    }
                }
            }
            if current_flash == flashing.len() {
                break;
            }
        }

        for &pos in flashing.iter() {
            map.entry(pos).and_modify(|oct| *oct = 0);
        }

        flashes += flashing.len();
    }

    println!("{}", flashes);
}
