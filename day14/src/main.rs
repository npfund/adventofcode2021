use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
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
