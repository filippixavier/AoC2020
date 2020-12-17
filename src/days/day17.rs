use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

type HyperCoordinate = (isize, isize, isize);
#[derive(PartialEq)]
enum ConwayStatus {
    Active,
    Inactive,
}

fn prepare_input(input: String) -> HashMap<HyperCoordinate, ConwayStatus> {
    let mut cube = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, state) in line.trim().chars().enumerate() {
            let coordinate = (x as isize, y as isize, 0isize);
            let conway_state = match state {
                '.' => ConwayStatus::Inactive,
                '#' => ConwayStatus::Active,
                _ => unreachable!(),
            };
            cube.insert(coordinate, conway_state);
        }
    }

    cube
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut states = prepare_input(fs::read_to_string(Path::new("./data/day17.txt"))?);

    let neighbors = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, 1, 1),
        (0, 1, -1),
        (0, -1, 0),
        (0, -1, 1),
        (0, -1, -1),
        (1, 0, 0),
        (1, 0, 1),
        (1, 0, -1),
        (1, 1, 0),
        (1, 1, 1),
        (1, 1, -1),
        (1, -1, 0),
        (1, -1, 1),
        (1, -1, -1),
        (-1, 0, 0),
        (-1, 0, 1),
        (-1, 0, -1),
        (-1, 1, 0),
        (-1, 1, 1),
        (-1, 1, -1),
        (-1, -1, 0),
        (-1, -1, 1),
        (-1, -1, -1),
    ];

    for _ in 0..6 {
        let mut next_states = HashMap::new();
        let mut heat_map: HashMap<HyperCoordinate, usize> = HashMap::new();

        for ((x, y, z), state) in states.iter() {
            if let ConwayStatus::Active = state {
                for (nx, ny, nz) in neighbors.iter() {
                    let neighbor_coordinate = (x + nx, y + ny, z + nz);
                    if let Some(count) = heat_map.get_mut(&neighbor_coordinate) {
                        *count += 1;
                    } else {
                        heat_map.insert(neighbor_coordinate, 1);
                    }
                }
            }
        }

        for (coordinate, count) in heat_map.iter() {
            let cell_status = states.get(coordinate).unwrap_or(&ConwayStatus::Inactive);
            let next_state = match (count, cell_status) {
                (2, ConwayStatus::Active)
                | (3, ConwayStatus::Active)
                | (3, ConwayStatus::Inactive) => ConwayStatus::Active,
                _ => ConwayStatus::Inactive,
            };

            next_states.insert(*coordinate, next_state);
        }

        states = next_states;
    }

    let active_cells = states
        .values()
        .filter(|&state| *state == ConwayStatus::Active)
        .count();

    println!("After 6 cycles there is {} active cells", active_cells);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
