use std::error::Error;
use std::fs;
use std::path::Path;

enum Operation {
    Plus,
    Minus,
    Mul,
}

use Operation::*;

fn prepare_input(input: String) -> Vec<Vec<String>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .fold(String::new(), |acc, element| match element {
                    '(' => format!("{}{} ", acc, element),
                    ')' => format!("{} {}", acc, element),
                    _ => format!("{}{}", acc, element),
                })
                .split(' ')
                .map(String::from)
                .collect()
        })
        .collect()
}

fn do_recursive_math(math: &[String], position: &mut usize) -> i64 {
    let mut result = 0;
    let mut op = Plus;
    loop {
        let element = if let Some(value) = math.get(*position) {
            value.as_str()
        } else {
            return result;
        };
        *position += 1;
        match element {
            "+" => {
                op = Plus;
            }
            "-" => {
                op = Minus;
            }
            "*" => {
                op = Mul;
            }
            "(" => {
                let rvalue = do_recursive_math(math, position);
                match op {
                    Plus => {
                        result += rvalue;
                    }
                    Minus => {
                        result -= rvalue;
                    }
                    Mul => {
                        result *= rvalue;
                    }
                }
            }
            ")" => {
                return result;
            }
            _ => {
                let rvalue = element.parse::<i64>().unwrap();
                match op {
                    Plus => {
                        result += rvalue;
                    }
                    Minus => {
                        result -= rvalue;
                    }
                    Mul => {
                        result *= rvalue;
                    }
                }
            }
        }
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let maths = prepare_input(fs::read_to_string(Path::new("./data/day18.txt"))?);
    let results: i64 = maths
        .iter()
        .map(|line| do_recursive_math(line, &mut 0))
        .sum();

    println!("Sum of operations is: {}", results);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
