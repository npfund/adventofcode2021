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
        for (x, digit) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert((x, y), digit as usize);
        }
    }

    let path = a_star(&map, (0, 0), (99, 99)).unwrap();
    let score =
        path.iter().fold(0, |acc, pos| acc + map.get(pos).unwrap()) - map.get(&(0, 0)).unwrap();

    println!("{score}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut map = HashMap::new();
    for (y, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        for (x, digit) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            for super_y in 0..5 {
                for super_x in 0..5 {
                    let value = ((digit + super_y + super_x - 1) % 9) + 1;
                    map.insert(
                        (x + (100 * super_x as usize), y + (100 * super_y as usize)),
                        value as usize,
                    );
                }
            }
        }
    }

    let path = a_star(&map, (0, 0), (499, 499)).unwrap();
    let score =
        path.iter().fold(0, |acc, pos| acc + map.get(pos).unwrap()) - map.get(&(0, 0)).unwrap();

    println!("{score}");
}

// flagrantly translated from the wikipedia pseudocode
fn a_star(
    map: &HashMap<(usize, usize), usize>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Result<Vec<(usize, usize)>, ()> {
    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0_usize);

    let mut f_score = HashMap::new();
    f_score.insert(start, distance(start, goal));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .fold(((0, 0), usize::MAX), |max, current| {
                if let Some(&score) = f_score.get(current) {
                    if score < max.1 {
                        (*current, score)
                    } else {
                        max
                    }
                } else {
                    max
                }
            })
            .0;

        if current == goal {
            return Ok(reconstruct_path(came_from, current));
        }

        open_set.remove(&current);
        let mut neighbors = Vec::new();

        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }

        if current.0 < goal.0 {
            neighbors.push((current.0 + 1, current.1));
        }

        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }

        if current.1 < goal.1 {
            neighbors.push((current.0, current.1 + 1));
        }

        for neighbor in neighbors {
            if let Some(&neighbor_value) = map.get(&neighbor) {
                let tentative_g_score =
                    g_score.get(&current).copied().unwrap_or(usize::MAX) + neighbor_value;
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(usize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + distance(neighbor, goal));
                    open_set.insert(neighbor);
                }
            }
        }
    }

    Err(())
}

fn distance(start: (usize, usize), end: (usize, usize)) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    mut current: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![current];
    while let Some(&next) = came_from.get(&current) {
        current = next;
        path.push(current);
    }

    path.reverse();

    path
}
