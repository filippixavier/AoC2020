use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

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

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
