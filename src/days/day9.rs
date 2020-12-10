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

fn find_anomaly(input: &[usize]) -> (usize, usize) {
    let mut position = RANGE;
    let mut preamble: HashSet<usize> = input.iter().take(RANGE).copied().collect();

    while position < input.len() {
        let current = input[position];
        let mut is_valid = false;
        for preamble_value in preamble.iter() {
            if *preamble_value >= current {
                continue;
            }
            let value_to_check = current.max(*preamble_value) - current.min(*preamble_value);
            if preamble.contains(&value_to_check) {
                is_valid = true;
                break;
            }
        }

        if is_valid {
            preamble.remove(&input[position - RANGE]);
            preamble.insert(current);
            position += 1;
        } else {
            return (current, position);
        }
    }

    (0, 0)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let values = prepare_input(fs::read_to_string(Path::new("./data/day9.txt"))?);

    let (anomaly, position) = find_anomaly(&values);

    println!(
        "Value {} at position {} doesn't respect the code!",
        anomaly, position
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::cmp::Ordering::*;
    let values = prepare_input(fs::read_to_string(Path::new("./data/day9.txt"))?);

    let (anomaly, position) = find_anomaly(&values);

    let candidates = &values[..position];

    let mut range = (0, 1);
    let mut sum = candidates[0] + candidates[1];

    loop {
        match sum.cmp(&anomaly) {
            Greater => {
                sum -= candidates[range.0];
                range.0 += 1;
                if range.0 > range.1 {
                    range.1 = range.0;
                    sum = candidates[range.0];
                }
            }
            Less => {
                range.1 += 1;
                sum += candidates[range.1];
            }
            Equal => {
                let encryption_weakpoint = &candidates[range.0..=range.1];
                let (min, max) = (
                    encryption_weakpoint.iter().min().unwrap(),
                    encryption_weakpoint.iter().max().unwrap(),
                );
                println!("Encryption weakpoint value is {}", *min + *max);
                break;
            }
        }
    }

    Ok(())
}
