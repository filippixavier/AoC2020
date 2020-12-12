use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Forward(isize),
    Left(isize),
    Right(isize),
}

struct Ship {
    facing: isize,
    coordinates: (isize, isize),
    waypoint_coordinates: (isize, isize),
    movements: Vec<Direction>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: 90,
            coordinates: (0, 0),
            waypoint_coordinates: (1, 10),
            movements: vec![],
        }
    }

    fn sail(&mut self, movement: Direction) {
        use Direction::*;
        match movement {
            North(value) => {
                self.coordinates.0 += value;
            }
            East(value) => {
                self.coordinates.1 += value;
            }
            South(value) => {
                self.coordinates.0 -= value;
            }
            West(value) => {
                self.coordinates.1 -= value;
            }
            Forward(value) => {
                let (coordinate, mult): (&mut isize, isize) = match self.facing {
                    0 => (&mut self.coordinates.0, 1),
                    90 => (&mut self.coordinates.1, 1),
                    180 => (&mut self.coordinates.0, -1),
                    270 => (&mut self.coordinates.1, -1),
                    _ => unreachable!(),
                };

                *coordinate += value * mult;
            }
            Left(value) => {
                self.facing = if value > self.facing {
                    360 + self.facing - value
                } else {
                    self.facing - value
                };
            }
            Right(value) => {
                self.facing = (self.facing + value) % 360;
            }
        }
    }

    fn rotate_waypoint_coordinates(
        &self,
        waypoint_coordinates: (isize, isize),
        orientation: isize,
    ) -> (isize, isize) {
        match orientation {
            90 => (-waypoint_coordinates.1, waypoint_coordinates.0),
            180 => (-waypoint_coordinates.0, -waypoint_coordinates.1),
            270 => (waypoint_coordinates.1, -waypoint_coordinates.0),
            _ => unreachable!(),
        }
    }

    fn sail_with_waypoint(&mut self, movement: Direction) {
        use Direction::*;
        let mut rotation = None;
        match movement {
            North(value) => {
                self.waypoint_coordinates.0 += value;
            }
            East(value) => {
                self.waypoint_coordinates.1 += value;
            }
            South(value) => {
                self.waypoint_coordinates.0 -= value;
            }
            West(value) => {
                self.waypoint_coordinates.1 -= value;
            }
            Forward(value) => {
                self.coordinates = (
                    self.coordinates.0 + self.waypoint_coordinates.0 * value,
                    self.coordinates.1 + self.waypoint_coordinates.1 * value,
                );
            }
            Left(value) => {
                rotation = Some(360 - value);
            }
            Right(value) => {
                rotation = Some(value);
            }
        }
        if let Some(rotate) = rotation {
            self.waypoint_coordinates =
                self.rotate_waypoint_coordinates(self.waypoint_coordinates, rotate);
        }
    }

    fn navigate(mut self) -> Self {
        let movements = self.movements;
        self.movements = vec![];
        for movement in movements {
            self.sail(movement);
        }
        self
    }

    fn navigate_using_waypoint(mut self) -> Self {
        let movements = self.movements;
        self.movements = vec![];
        for movement in movements {
            self.sail_with_waypoint(movement);
        }
        self
    }
}

fn prepare_input(input: String) -> Ship {
    let mut ship = Ship::new();

    for line in input.trim().lines() {
        let mut instruction = line.trim().chars();
        let direction = instruction.next().unwrap();
        let count = instruction.collect::<String>().parse::<isize>().unwrap();

        let movement = match direction {
            'N' => Direction::North(count),
            'E' => Direction::East(count),
            'S' => Direction::South(count),
            'W' => Direction::West(count),
            'F' => Direction::Forward(count),
            'L' => Direction::Left(count),
            'R' => Direction::Right(count),
            _ => unreachable!(),
        };

        ship.movements.push(movement);
    }

    ship
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut ship = prepare_input(fs::read_to_string(Path::new("./data/day12.txt"))?);
    ship = ship.navigate();
    println!(
        "Ship navigated {} miles!",
        ship.coordinates.0.abs() + ship.coordinates.1.abs()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut ship = prepare_input(fs::read_to_string(Path::new("./data/day12.txt"))?);
    ship = ship.navigate_using_waypoint();
    println!(
        "Ship navigated {} miles thanks to the waypoint!",
        ship.coordinates.0.abs() + ship.coordinates.1.abs()
    );
    Ok(())
}
