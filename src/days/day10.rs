use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<usize> {
    // Copied the parse function here:
    // https://gitlab.com/mboehnke/aoc-2020/-/blob/master/aoc-2020-09/src/solution.rs
    input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut adapters = prepare_input(fs::read_to_string(Path::new("./data/day10.txt"))?);

    adapters.sort_unstable();

    let result = adapters
        .iter()
        .scan(0usize, |previous, &adapter| {
            let diff = adapter - *previous;
            let mut result = Some(0);

            // println!("{} {} {}", previous, adapter, diff);
            *previous = adapter;

            if diff == 1 || diff == 3 {
                result = Some(diff)
            }

            result
        })
        .fold((0, 0), |acc, x| {
            let mut next_acc = acc;
            if x == 3 {
                next_acc.1 += 1;
            } else if x == 1 {
                next_acc.0 += 1;
            }

            next_acc
        });

    println!("Total jolting difference is {}", result.0 * (result.1 + 1));

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
