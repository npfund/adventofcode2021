use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut map = HashMap::new();
    for (y, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        for (x, height) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((x as i32, y as i32), height);
        }
    }

    let mut basins = Vec::new();
    for ((x, y), &height) in &map {
        let up = *map.get(&(*x, y - 1)).unwrap_or(&9);
        let right = *map.get(&(x + 1, *y)).unwrap_or(&9);
        let down = *map.get(&(*x, y + 1)).unwrap_or(&9);
        let left = *map.get(&(x - 1, *y)).unwrap_or(&9);

        if height < up && height < right && height < down && height < left {
            let mut basin = HashSet::new();
            flood(&map, &mut basin, *x, *y);
            basins.push(basin);
        }
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    let total = basins[0].len() * basins[1].len() * basins[2].len();

    println!("{}", total);
}

fn flood(map: &HashMap<(i32, i32), u32>, basin: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    basin.insert((x, y));

    let up = *map.get(&(x, y - 1)).unwrap_or(&9);
    if up < 9 && !basin.contains(&(x, y - 1)) {
        flood(map, basin, x, y - 1);
    }

    let right = *map.get(&(x + 1, y)).unwrap_or(&9);
    if right < 9 && !basin.contains(&(x + 1, y)) {
        flood(map, basin, x + 1, y);
    }

    let down = *map.get(&(x, y + 1)).unwrap_or(&9);
    if down < 9 && !basin.contains(&(x, y + 1)) {
        flood(map, basin, x, y + 1);
    }

    let left = *map.get(&(x - 1, y)).unwrap_or(&9);
    if left < 9 && !basin.contains(&(x - 1, y)) {
        flood(map, basin, x - 1, y);
    }
}
