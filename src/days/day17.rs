use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

type SuperCoordinates = (isize, isize, isize);
type HyperCoordinates = (SuperCoordinates, isize);

#[derive(PartialEq)]
enum ConwayStatus {
    Active,
    Inactive,
}

static NEIGHBORS: [(isize, isize, isize); 27] = [
    (0, 0, 0),
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

fn prepare_input(input: String) -> HashMap<SuperCoordinates, ConwayStatus> {
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
    for _ in 0..6 {
        let mut next_states = HashMap::new();
        let mut heat_map: HashMap<SuperCoordinates, usize> = HashMap::new();

        for ((x, y, z), state) in states.iter() {
            if let ConwayStatus::Active = state {
                for (nx, ny, nz) in NEIGHBORS.iter().skip(1) {
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

fn to_hyper_mode(
    input: HashMap<SuperCoordinates, ConwayStatus>,
) -> HashMap<HyperCoordinates, ConwayStatus> {
    let mut result = HashMap::new();

    for (coordinate, status) in input {
        result.insert((coordinate, 0), status);
    }

    result
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut states = to_hyper_mode(prepare_input(fs::read_to_string(Path::new(
        "./data/day17.txt",
    ))?));

    for _ in 0..6 {
        let mut next_states = HashMap::new();
        let mut heat_map: HashMap<HyperCoordinates, usize> = HashMap::new();

        for (((x, y, z), t), state) in states.iter() {
            if let ConwayStatus::Active = state {
                for i in -1..=1 {
                    for (nx, ny, nz) in NEIGHBORS.iter() {
                        if i == 0 && *nx == 0 && *ny == 0 && *nz == 0 {
                            continue;
                        }
                        let temporal_neighbor_coordinate = ((x + nx, y + ny, z + nz), t + i);
                        if let Some(count) = heat_map.get_mut(&temporal_neighbor_coordinate) {
                            *count += 1;
                        } else {
                            heat_map.insert(temporal_neighbor_coordinate, 1);
                        }
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
