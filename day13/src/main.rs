use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut paper = HashSet::new();
    let mut folds: Vec<(char, i32)> = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some((x, y)) = line.split_once(',') {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            paper.insert((x, y));
        } else if let Some((left, line)) = line.split_once('=') {
            let axis = left.chars().last().unwrap();
            let position = line.parse::<i32>().unwrap();
            folds.push((axis, position));
        }
    }

    for (axis, position) in folds {
        let mut folded = paper.clone();
        match axis {
            'x' => {
                for &(x, y) in &paper {
                    folded.remove(&(x, y));
                    let x = if x > position {
                        position - (x - position)
                    } else {
                        x
                    };
                    folded.insert((x, y));
                }
            }
            'y' => {
                for &(x, y) in &paper {
                    folded.remove(&(x, y));
                    let y = if y > position {
                        position - (y - position)
                    } else {
                        y
                    };
                    folded.insert((x, y));
                }
            }
            _ => panic!(),
        }

        paper = folded;
        break;
    }

    println!("{}", paper.len());
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut paper = HashSet::new();
    let mut folds: Vec<(char, i32)> = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some((x, y)) = line.split_once(',') {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            paper.insert((x, y));
        } else if let Some((left, line)) = line.split_once('=') {
            let axis = left.chars().last().unwrap();
            let position = line.parse::<i32>().unwrap();
            folds.push((axis, position));
        }
    }

    for (axis, position) in folds {
        let mut folded = paper.clone();
        match axis {
            'x' => {
                for &(x, y) in &paper {
                    folded.remove(&(x, y));
                    let x = if x > position {
                        position - (x - position)
                    } else {
                        x
                    };
                    folded.insert((x, y));
                }
            }
            'y' => {
                for &(x, y) in &paper {
                    folded.remove(&(x, y));
                    let y = if y > position {
                        position - (y - position)
                    } else {
                        y
                    };
                    folded.insert((x, y));
                }
            }
            _ => panic!(),
        }

        paper = folded;
    }

    for y in 0..50 {
        for x in 0..50 {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
