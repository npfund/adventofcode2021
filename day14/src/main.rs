use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut template: Vec<char> = Vec::new();
    let mut rules = HashMap::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some((pattern, insert)) = line.split_once(" -> ") {
            let pattern = (
                pattern.chars().nth(0).unwrap(),
                pattern.chars().nth(1).unwrap(),
            );
            let insert = insert.chars().nth(0).unwrap();

            rules.insert(pattern, insert);
        } else if !line.is_empty() {
            template = line.chars().collect();
        }
    }

    for _ in 0..10 {
        let stream = template
            .iter()
            .cloned()
            .zip(template.iter().skip(1).cloned())
            .enumerate();
        let mut new = template.clone();
        let mut inserted = 0;
        for (position, pair) in stream {
            if let Some(insert) = rules.get(&pair) {
                new.insert(position + 1 + inserted, insert.clone());
                inserted += 1;
            }
        }

        template = new;
    }

    let counts = template.iter().fold(HashMap::new(), |mut acc, &char| {
        let entry = acc.entry(char).or_insert(0);
        *entry += 1;

        acc
    });

    let diff = counts.values().max().unwrap() - counts.values().min().unwrap();

    println!("{}", diff);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut counts = HashMap::new();
    let mut rules = HashMap::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some((pattern, insert)) = line.split_once(" -> ") {
            let pattern = (
                pattern.chars().nth(0).unwrap(),
                pattern.chars().nth(1).unwrap(),
            );
            let insert = insert.chars().nth(0).unwrap();

            rules.insert(pattern, insert);
        } else if !line.is_empty() {
            let template = line.chars().collect::<Vec<_>>();
            let stream = template
                .iter()
                .cloned()
                .zip(template.iter().skip(1).cloned());
            for pair in stream {
                let entry = counts.entry(pair).or_insert(0_u64);
                *entry += 1;
            }
        }
    }

    for _ in 0..40 {
        let mut new_counts = counts.clone();
        for (pair, &insert) in &rules {
            if let Some(&source) = counts.get(&pair) {
                let left = new_counts.entry((pair.0, insert)).or_insert(0);
                *left += source;
                let right = new_counts.entry((insert, pair.1)).or_insert(0);
                *right += source;
                let new_source = new_counts.entry(*pair).or_insert(0);
                *new_source -= source;
            }
        }
        counts = new_counts;
    }

    let sums = counts
        .iter()
        .fold(HashMap::new(), |mut acc, (pair, &value)| {
            let left = acc.entry(pair.0).or_insert(0);
            *left += value;
            let right = acc.entry(pair.1).or_insert(0);
            *right += value;

            acc
        });

    let diff = sums.values().max().unwrap() - sums.values().min().unwrap();

    println!("{}", (diff / 2) + 1);
}
