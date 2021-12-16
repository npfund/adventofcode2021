use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let cave = file
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let l = line.split_once('-').unwrap();
            (l.0.to_owned(), l.1.to_owned())
        })
        .collect::<Vec<_>>();

    let paths = step(&cave, vec!["start".to_string()]);
    let complete_paths = paths
        .iter()
        .filter(|path| path.last().unwrap() == "end")
        .count();

    println!("{}", complete_paths);
}

fn step(cave: &[(String, String)], path: Vec<String>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let position = path.last().unwrap();
    if position == "end" {
        return paths;
    }
    for s in cave {
        let destination = if s.0 == *position {
            &s.1
        } else if s.1 == *position {
            &s.0
        } else {
            continue;
        };

        if *destination == destination.to_ascii_lowercase() && path.contains(destination) {
            continue;
        }

        let mut new_path = path.to_vec();
        new_path.push(destination.clone());
        paths.push(new_path.clone());

        paths.extend(step(cave, new_path));
    }

    paths
}
