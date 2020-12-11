use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
enum Seat {
    Occupied,
    Empty,
    Ground,
}

use Seat::*;

type Seats = HashMap<(isize, isize), Seat>;

#[derive(Clone, Debug)]
struct SeatsMap {
    seats: Seats,
    cols: usize,
    lines: usize,
}

impl SeatsMap {
    fn get_signature(&self) -> String {
        let mut result = String::new();
        for i in 0..self.lines {
            for j in 0..self.cols {
                result += match self.seats.get(&(i as isize, j as isize)).unwrap() {
                    Occupied => "#",
                    Empty => "L",
                    Ground => ".",
                };
            }
        }
        result
    }

    fn count_neighbors(&self, line: isize, col: isize) -> (usize, usize, usize) {
        let mut count_empty = 0;
        let mut count_occupied = 0;
        let mut count_ground = 0;
        let neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for neighbor in &neighbors {
            if let Some(tile) = self.seats.get(&(line + neighbor.0, col + neighbor.1)) {
                match tile {
                    Empty => count_empty += 1,
                    Ground => count_ground += 1,
                    Occupied => count_occupied += 1,
                }
            }
        }

        (count_empty, count_occupied, count_ground)
    }

    fn count_total(&self) -> (usize, usize, usize) {
        let mut count_empty = 0;
        let mut count_occupied = 0;
        let mut count_ground = 0;

        for i in 0..self.lines {
            for j in 0..self.cols {
                match self.seats.get(&(i as isize, j as isize)).unwrap() {
                    Occupied => count_occupied += 1,
                    Empty => count_empty += 1,
                    Ground => count_ground += 1,
                };
            }
        }

        (count_empty, count_occupied, count_ground)
    }
}

fn prepare_input(input: String) -> SeatsMap {
    let seats = HashMap::new();
    let mut result = SeatsMap {
        seats,
        cols: 0,
        lines: 0,
    };

    result.lines = input.trim().lines().count();

    for (line_no, line) in input.trim().lines().enumerate() {
        result.cols = line.trim().chars().count();
        for (col_no, seat) in line.trim().chars().enumerate() {
            let seat = match seat {
                'L' => Empty,
                '.' => Ground,
                _ => unreachable!(),
            };
            result
                .seats
                .insert((line_no as isize, col_no as isize), seat);
        }
    }

    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut seats = prepare_input(fs::read_to_string(Path::new("./data/day11.txt"))?);
    let mut previous_sig = seats.get_signature();
    loop {
        let mut next_seats = seats.seats.clone();

        for i in 0..seats.lines {
            for j in 0..seats.cols {
                let line = i as isize;
                let col = j as isize;

                match next_seats.get(&(line, col)).unwrap() {
                    Empty => {
                        let (_, count_occupied, _) = seats.count_neighbors(line, col);
                        if count_occupied == 0 {
                            next_seats.insert((line, col), Occupied);
                        }
                    }
                    Occupied => {
                        let (_, count_occupied, _) = seats.count_neighbors(line, col);
                        if count_occupied >= 4 {
                            next_seats.insert((line, col), Empty);
                        }
                    }
                    Ground => {
                        continue;
                    }
                }
            }
        }

        seats.seats = next_seats;
        let signature = seats.get_signature();
        if signature == previous_sig {
            break;
        }
        previous_sig = signature;
    }

    let (_, count_occupied, _) = seats.count_total();

    println!(
        "Once everyone is settled, there is {} occupied",
        count_occupied
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
