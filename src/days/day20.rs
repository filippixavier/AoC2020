use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
struct Tile {
    id: i64,
    top_border: String,
    right_border: String,
    bottom_border: String,
    left_border: String,
    image: String,
}

impl Tile {
    fn new(raw: String) -> Tile {
        let mut tile = Tile {
            id: 0,
            top_border: String::new(),
            right_border: String::new(),
            bottom_border: String::new(),
            left_border: String::new(),
            image: String::new(),
        };

        let mut raw_lines = raw.lines();
        let raw_id = raw_lines.next().unwrap().to_string();

        tile.id = raw_id[5..9].parse().unwrap();
        tile.image = raw_lines.clone().collect();

        for (line_no, line) in raw_lines.enumerate() {
            if line_no == 0 {
                tile.top_border = line.to_string();
            }
            let line_chars = line.chars().collect::<Vec<_>>();

            tile.bottom_border = line.to_string();
            tile.left_border.push(*line_chars.first().unwrap());
            tile.right_border.push(*line_chars.last().unwrap());
        }

        tile
    }

    fn matching_borders(&self, other: &Self) -> usize {
        let mut count = 0;
        let borders = [
            self.top_border.to_string(),
            self.right_border.to_string(),
            self.bottom_border.to_string(),
            self.left_border.to_string(),
        ];
        let other_borders = [
            other.top_border.to_string(),
            other.right_border.to_string(),
            other.bottom_border.to_string(),
            other.left_border.to_string(),
        ];

        for border in borders.iter() {
            for other_border in other_borders.iter() {
                if *border == *other_border {
                    count += 1;
                }
            }
        }
        if count == 0 {
            let reverse_borders = borders
                .iter()
                .map(|elem| elem.chars().rev().collect())
                .collect::<Vec<String>>();
            for border in reverse_borders.iter() {
                for other_border in other_borders.iter() {
                    if *border == *other_border {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn prepare_input(input: String) -> Vec<Tile> {
    let mut tiles = vec![];

    for bloc in input.trim().split("\r\n\r\n") {
        tiles.push(Tile::new(bloc.to_string()));
    }

    tiles
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let tiles = prepare_input(fs::read_to_string(Path::new("./data/day20.txt"))?);
    let mut result = 1i64;

    for tile in tiles.iter() {
        let mut border_count = 0;
        for tile_2 in tiles.iter() {
            if tile.id == tile_2.id {
                continue;
            }
            border_count += tile.matching_borders(&tile_2);
        }

        if border_count == 2 {
            result *= tile.id as i64;
        }
    }

    println!("Result from corner images id: {}", result);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
