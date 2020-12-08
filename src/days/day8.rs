use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

// Just keeping these lines to remember how to return a closure
type Operation = Box<dyn Fn(isize, usize) -> (isize, usize)>;

fn prepare_input(input: String) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];

    for line in input.trim().lines() {
        let code = line.split(' ').collect::<Vec<_>>();
        let value_increment = code[1].parse::<isize>().unwrap();
        let func = match code[0] {
            "nop" => returns_closure(0, 1),
            "acc" => returns_closure(value_increment, 1),
            "jmp" => returns_closure(0, value_increment),
            _ => unreachable!(),
        };
        operations.push(func);
    }

    operations
}

fn returns_closure(acc_increment: isize, line_increment: isize) -> Operation {
    use std::convert::TryInto;
    Box::new(move |acc, line| {
        let next_line = line as isize + line_increment;
        (acc + acc_increment, next_line.try_into().unwrap_or(0))
    })
}

#[derive(Debug, Clone)]
struct Code {
    instruction: Instruction,
    arg_value: isize,
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    NOOP,
    JMP,
    ACC,
}

fn prepare_input_2(input: String) -> Vec<Code> {
    let mut codes = vec![];

    for line in input.trim().lines() {
        let code = line.split(' ').collect::<Vec<_>>();
        let instruction = match code[0] {
            "nop" => Instruction::NOOP,
            "acc" => Instruction::ACC,
            "jmp" => Instruction::JMP,
            _ => unreachable!(),
        };
        let arg_value = code[1].parse::<isize>().unwrap();
        codes.push(Code {
            instruction,
            arg_value,
        })
    }

    codes
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let code = prepare_input(fs::read_to_string(Path::new("./data/day8.txt"))?);
    let mut acc = 0;
    let mut line = 0;
    let mut explored_line = HashSet::new();

    while explored_line.insert(line) {
        let next_step = code[line](acc, line);
        acc = next_step.0;
        line = next_step.1;
    }

    println!(
        "First repeated line of code: {} for accumulator value of {}",
        line, acc
    );

    Ok(())
}

fn exec(codes: Vec<Code>) -> Option<isize> {
    use Instruction::*;

    let mut acc = 0;
    let mut line = 0;
    let mut visited_line = HashSet::new();

    loop {
        if !visited_line.insert(line) {
            return None;
        }
        if let Some(loc) = codes.get(line) {
            match loc.instruction {
                ACC => {
                    acc += loc.arg_value;
                    line += 1;
                }
                NOOP => {
                    line += 1;
                }
                JMP => {
                    let next_line = ((line as isize) + loc.arg_value) as usize;
                    line = next_line;
                }
            }
        } else {
            return Some(acc);
        }
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use Instruction::*;
    let codes = prepare_input_2(fs::read_to_string(Path::new("./data/day8.txt"))?);

    for (line, loc) in codes.iter().enumerate() {
        if loc.instruction == ACC {
            continue;
        }
        let mut fixed_code = codes.clone();
        fixed_code[line] = match loc.instruction {
            JMP => Code {
                instruction: NOOP,
                arg_value: loc.arg_value,
            },
            NOOP => Code {
                instruction: JMP,
                arg_value: loc.arg_value,
            },
            ACC => unreachable!(),
        };

        if let Some(acc_value) = exec(fixed_code) {
            println!("Accumulator value after successful boot is: {}", acc_value);
            break;
        }
    }

    Ok(())
}
