use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Side {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
use Side::*;

struct HexaTile {
    x: isize,
    y: isize,
    z: isize,
}

type Coordinate = (isize, isize, isize);

impl HexaTile {
    fn move_position(&mut self, side: Side) {
        let next_position = get_next_position((self.x, self.y, self.z), side);
        self.x = next_position.0;
        self.y = next_position.1;
        self.z = next_position.2;
    }
}

fn get_next_position(starting_pos: Coordinate, side: Side) -> Coordinate {
    let mut next_position = starting_pos;
    match side {
        East => {
            next_position.0 += 1;
            next_position.1 -= 1;
        }
        SouthEast => {
            next_position.1 -= 1;
            next_position.2 += 1;
        }
        SouthWest => {
            next_position.0 -= 1;
            next_position.2 += 1;
        }
        West => {
            next_position.0 -= 1;
            next_position.1 += 1;
        }
        NorthWest => {
            next_position.1 += 1;
            next_position.2 -= 1;
        }
        NorthEast => {
            next_position.0 += 1;
            next_position.2 -= 1;
        }
    }

    next_position
}

fn prepare_input(input: String) -> Vec<Vec<Side>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut result = vec![];
            let mut two_parts = None;
            for val in line.trim().chars() {
                match val {
                    'e' => {
                        let value_to_push = if let Some(north) = two_parts {
                            two_parts = None;
                            if north {
                                NorthEast
                            } else {
                                SouthEast
                            }
                        } else {
                            East
                        };
                        result.push(value_to_push);
                    }
                    'w' => {
                        let value_to_push = if let Some(north) = two_parts {
                            two_parts = None;
                            if north {
                                NorthWest
                            } else {
                                SouthWest
                            }
                        } else {
                            West
                        };
                        result.push(value_to_push);
                    }
                    'n' => {
                        two_parts = Some(true);
                    }
                    's' => {
                        two_parts = Some(false);
                    }
                    _ => unreachable!(),
                }
            }
            result
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::collections::HashMap;
    let mut black_tiles = HashMap::<Coordinate, bool>::new();

    let tiles = prepare_input(fs::read_to_string(Path::new("./data/day24.txt"))?);

    for instructions in tiles {
        let mut tile = HexaTile { x: 0, y: 0, z: 0 };
        for movement in instructions {
            tile.move_position(movement);
        }
        let coordinate = (tile.x, tile.y, tile.z);
        if let Some(color) = black_tiles.get_mut(&coordinate) {
            *color = !(*color);
        } else {
            black_tiles.insert(coordinate, true);
        }
    }

    println!(
        "Number of black tiles: {}",
        black_tiles.values().filter(|&v| *v).count()
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::collections::HashMap;
    let mut black_tiles = HashMap::<Coordinate, bool>::new();
    let directions = [East, SouthEast, SouthWest, West, NorthWest, NorthEast];

    let tiles = prepare_input(fs::read_to_string(Path::new("./data/day24.txt"))?);

    for instructions in tiles {
        let mut tile = HexaTile { x: 0, y: 0, z: 0 };
        for movement in instructions {
            tile.move_position(movement);
        }
        let coordinate = (tile.x, tile.y, tile.z);
        if let Some(color) = black_tiles.get_mut(&coordinate) {
            *color = !(*color);
        } else {
            black_tiles.insert(coordinate, true);
        }
    }

    for _ in 1..=100 {
        let mut flipped_tiles = HashMap::new();

        let mut tagged_tiles: HashMap<Coordinate, usize> = HashMap::new();

        for (coordinate, is_black) in black_tiles.iter() {
            if !is_black {
                continue;
            }

            tagged_tiles.entry(*coordinate).or_insert(0);

            for neighbor in directions.iter() {
                let neigh_pos = get_next_position(*coordinate, *neighbor);
                *tagged_tiles.entry(neigh_pos).or_insert(0) += 1;
            }
        }

        for (coordinate, number) in tagged_tiles.iter() {
            let is_tile_black = if let Some(is_black) = black_tiles.get(coordinate) {
                *is_black
            } else {
                false
            };

            let will_be_black = match (*number, is_tile_black) {
                (i, true) if i == 1 || i == 2 => true,
                (2, false) => true,
                _ => false,
            };

            if will_be_black {
                flipped_tiles.insert(*coordinate, true);
            }
        }
        black_tiles = flipped_tiles;
    }

    println!(
        "Number of black tiles: {}",
        black_tiles.values().filter(|&v| *v).count()
    );

    Ok(())
}
