use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

#[derive(Eq, PartialEq)]
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
