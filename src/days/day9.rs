use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

static RANGE: usize = 25;

fn prepare_input(input: String) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|value| value.parse().unwrap())
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let values = prepare_input(fs::read_to_string(Path::new("./data/day9.txt"))?);
    let mut preamble: HashSet<usize> = values.iter().take(RANGE).copied().collect();

    let mut position = RANGE;

    while position < values.len() {
        let current = values[position];
        let mut is_valid = false;
        for preamble_value in preamble.iter() {
            if *preamble_value >= current {
                continue;
            }
            let value_to_check =
                cmp::max(current, *preamble_value) - cmp::min(current, *preamble_value);
            if preamble.contains(&value_to_check) {
                is_valid = true;
                break;
            }
        }

        if is_valid {
            preamble.remove(&values[position - RANGE]);
            preamble.insert(current);
            position += 1;
        } else {
            println!("Value {} doesn't respect the code!", current);
            break;
        }
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
