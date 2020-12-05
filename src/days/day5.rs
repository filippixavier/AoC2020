use std::error::Error;
use std::fs;
use std::path::Path;

type Seat = (usize, usize);

struct BoardingGroup {
    row_instructions: Vec<char>,
    col_instructions: Vec<char>,
}

impl BoardingGroup {
    fn find_seat(self) -> Seat {
        let mut row_range = (0, 127);
        let mut col_range = (0, 7);

        for r_instruction in self.row_instructions {
            let lower_range = row_range.0 + (row_range.1 - row_range.0) / 2;
            let higher_range = lower_range + 1;
            match r_instruction {
                'F' => {
                    row_range = (row_range.0, lower_range);
                }
                'B' => {
                    row_range = (higher_range, row_range.1);
                }
                _ => unreachable!(),
            }
        }

        let row = row_range.0;

        for c_instruction in self.col_instructions {
            let lower_range = col_range.0 + (col_range.1 - col_range.0) / 2;
            let higher_range = lower_range + 1;
            match c_instruction {
                'L' => {
                    col_range = (col_range.0, lower_range);
                }
                'R' => {
                    col_range = (higher_range, col_range.1);
                }
                _ => unreachable!(),
            }
        }

        let col = col_range.0;

        (row, col)
    }
}

fn prepare_input(input: String) -> Vec<BoardingGroup> {
    let mut groups = vec![];

    for line in input.trim().lines() {
        let group = BoardingGroup {
            row_instructions: (&line[..7]).chars().collect(),
            col_instructions: (&line[7..]).chars().collect(),
        };
        groups.push(group);
    }

    groups
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let boarding_passes = prepare_input(fs::read_to_string(Path::new("./data/day5.txt"))?);

    let highest_id = boarding_passes
        .into_iter()
        .map(|boarding| {
            let seat = boarding.find_seat();
            return seat.0 * 8 + seat.1;
        })
        .max()
        .unwrap();

    println!("Highest id is: {}", highest_id);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
