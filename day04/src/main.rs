use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut lines = file
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let choices = lines
        .drain(0..1)
        .collect::<String>()
        .split(',')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let mut boards = lines
        .as_slice()
        .chunks(5)
        .map(|chunk| {
            let mut board = HashMap::new();
            for (y, line) in chunk.iter().enumerate() {
                for (x, num) in line.split(' ').filter(|c| !c.is_empty()).enumerate() {
                    board.insert((x, y), (num, false));
                }
            }

            board
        })
        .collect::<Vec<_>>();

    for choice in choices {
        for board in boards.iter_mut() {
            for cell in board.iter_mut() {
                if cell.1 .0 == choice {
                    cell.1 .1 = true;
                }
            }

            let mut winner = false;
            for y in 0..5 {
                let mut row = true;
                for x in 0..5 {
                    row = row && board[&(x, y)].1
                }
                if row {
                    winner = true;
                    break;
                }
            }

            for x in 0..5 {
                let mut col = true;
                for y in 0..5 {
                    col = col && board[&(x, y)].1
                }
                if col {
                    winner = true;
                    break;
                }
            }

            if winner {
                let sum = board
                    .values()
                    .filter(|v| !v.1)
                    .map(|v| v.0.parse::<u64>().unwrap())
                    .sum::<u64>();

                println!("{}", sum * (choice.parse::<u64>().unwrap()));

                return;
            }
        }
    }
}
