use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Paren,
    Square,
    Curly,
    Angle,
    None,
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut score = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                '(' => stack.push(Token::Paren),
                '[' => stack.push(Token::Square),
                '{' => stack.push(Token::Curly),
                '<' => stack.push(Token::Angle),
                ')' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Paren {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Square {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Curly {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Angle {
                        score += 25137;
                        break;
                    }
                }
                _ => panic!("{}", char),
            }
        }
    }

    println!("{}", score);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut scores = Vec::new();
    'outer: for line in file.lines().map(|l| l.unwrap()) {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                '(' => stack.push(Token::Paren),
                '[' => stack.push(Token::Square),
                '{' => stack.push(Token::Curly),
                '<' => stack.push(Token::Angle),
                ')' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Paren {
                        continue 'outer;
                    }
                }
                ']' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Square {
                        continue 'outer;
                    }
                }
                '}' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Curly {
                        continue 'outer;
                    }
                }
                '>' => {
                    let top = stack.pop().unwrap_or(Token::None);
                    if top != Token::Angle {
                        continue 'outer;
                    }
                }
                _ => panic!("{}", char),
            }
        }

        let mut score: i64 = 0;
        for token in stack.iter().rev() {
            score *= 5;
            match token {
                Token::Paren => score += 1,
                Token::Square => score += 2,
                Token::Curly => score += 3,
                Token::Angle => score += 4,
                Token::None => panic!("none"),
            }
        }
        scores.push(score);
    }

    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
