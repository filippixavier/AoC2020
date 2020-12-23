use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|value| value.to_string().parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let cups = prepare_input(fs::read_to_string(Path::new("./data/day23.txt"))?);
    let mut cycled_cups: HashMap<usize, usize> = cups
        .iter()
        .enumerate()
        .map(|(index, value)| (*value - 1, cups[(index + 1) % cups.len()] - 1))
        .collect::<HashMap<usize, usize>>();

    let mut current_cup = cups[0] - 1;

    for _ in 0..100 {
        let pick1 = *cycled_cups.get(&current_cup).unwrap();
        let pick2 = *cycled_cups.get(&pick1).unwrap();
        let pick3 = *cycled_cups.get(&pick2).unwrap();
        let next = *cycled_cups.get(&pick3).unwrap();

        cycled_cups.insert(current_cup, next);

        let mut destination = current_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        while destination == pick1 || destination == pick2 || destination == pick3 {
            destination = destination.checked_sub(1).unwrap_or(cups.len() - 1);
        }

        cycled_cups.insert(pick3, *cycled_cups.get(&destination).unwrap());
        cycled_cups.insert(destination, pick1);
        current_cup = next;
    }

    current_cup = 0;

    let mut result = String::new();

    for _ in 0..cups.len() - 1 {
        current_cup = *cycled_cups.get(&current_cup).unwrap();
        result.push_str(&(current_cup + 1).to_string());
    }

    println!("Cup labels: {}", result);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let first_cups_order = prepare_input(fs::read_to_string(Path::new("./data/day23.txt"))?);
    let mut cups = (1..=1_000_000).collect::<Vec<usize>>();

    for (index, &value) in first_cups_order.iter().enumerate() {
        cups[index] = value;
    }

    let mut cycled_cups: HashMap<usize, usize> = cups
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, cups[(index + 1) % cups.len()]))
        .collect::<HashMap<usize, usize>>();
    let mut current_cup = cups[0];

    for _ in 0..10_000_000 {
        let pick1 = *cycled_cups.get(&current_cup).unwrap();
        let pick2 = *cycled_cups.get(&pick1).unwrap();
        let pick3 = *cycled_cups.get(&pick2).unwrap();
        let next = *cycled_cups.get(&pick3).unwrap();

        cycled_cups.insert(current_cup, next);

        let mut destination = if current_cup == 1 {
            1_000_000
        } else {
            current_cup - 1
        };
        while destination == pick1
            || destination == pick2
            || destination == pick3
            || destination == current_cup
        {
            destination = if destination == 1 {
                1_000_000
            } else {
                destination - 1
            };
        }

        cycled_cups.insert(pick3, *cycled_cups.get(&destination).unwrap());
        cycled_cups.insert(destination, pick1);
        current_cup = next;
    }
    let result;

    let first_cup = *cycled_cups.get(&1).unwrap();
    let second_cup = *cycled_cups.get(&first_cup).unwrap();

    result = first_cup as u64 * second_cup as u64;

    println!(
        "Cups are {}, {}, result is: {}",
        first_cup, second_cup, result
    );

    Ok(())
}
