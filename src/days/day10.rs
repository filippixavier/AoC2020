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
    use std::collections::HashMap;

    let mut adapters = prepare_input(fs::read_to_string(Path::new("./data/day10.txt"))?);
    adapters.sort_unstable();
    let mut memoize: HashMap<usize, u64> = HashMap::new();
    let mut count = 0;

    for adapter in adapters {
        count = 0;

        if adapter == 1 || adapter == 2 || adapter == 3 {
            count += 1;
        }

        if adapter > 1 {
            count += if let Some(stored) = memoize.get(&(adapter - 1)) {
                *stored
            } else {
                0
            }
        }

        if adapter > 2 {
            count += if let Some(stored) = memoize.get(&(adapter - 2)) {
                *stored
            } else {
                0
            }
        }

        if adapter > 3 {
            count += if let Some(stored) = memoize.get(&(adapter - 3)) {
                *stored
            } else {
                0
            }
        }

        memoize.insert(adapter, count);
    }

    println!("Total number of combinations: {}", count);

    Ok(())
}
