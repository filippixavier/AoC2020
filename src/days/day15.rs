use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::collections::HashMap;
    let mut input = prepare_input(fs::read_to_string(Path::new("./data/day15.txt"))?).into_iter();
    let mut positions = HashMap::<usize, usize>::new();

    let mut next_value = 0;
    let mut current_value = 0;

    for i in 1..=2020 {
        current_value = input.next().unwrap_or(next_value);
        if let Some(position) = positions.get_mut(&current_value) {
            next_value = i - *position;
            *position = i;
        } else {
            positions.insert(current_value, i);
            next_value = 0;
        }
    }
    println!("{}", current_value);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
