use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Tile {
    id: i64,
    image: Vec<Vec<char>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Side {
    Top,
    Right,
    Left,
    Bottom,
}

use Side::*;

impl Tile {
    fn new(raw: String) -> Tile {
        let mut tile = Tile {
            id: 0,
            image: vec![],
        };

        let mut raw_lines = raw.lines();
        let raw_id = raw_lines.next().unwrap().to_string();

        tile.id = raw_id[5..9].parse().unwrap();

        for line in raw_lines {
            tile.image.push(line.chars().collect());
        }

        tile
    }

    fn fit_to_other(&mut self, other: &Self) -> Option<Side> {
        let size = self.image.len();
        for count in 0..8 {
            let (mut top_match, mut right_match, mut bottom_match, mut left_match) =
                (true, true, true, true);
            for index in 0..size {
                top_match &= self.image[0][index] == other.image[size - 1][index];
                right_match &= self.image[index][size - 1] == other.image[index][0];
                bottom_match &= self.image[size - 1][index] == other.image[0][index];
                left_match &= self.image[index][0] == other.image[index][size - 1];
            }

            if top_match {
                return Some(Top);
            }
            if left_match {
                return Some(Left);
            }
            if bottom_match {
                return Some(Bottom);
            }
            if right_match {
                return Some(Right);
            }
            self.image = rotate_image(self.image.clone());
            if count == 3 {
                self.image = self.image.iter().cloned().rev().collect();
            }
        }
        None
    }

    fn get_borderless_picture(&self) -> Vec<Vec<char>> {
        let mut result = vec![];
        let size = self.image.len() - 1;

        for i in 1..size {
            let mut line = vec![];
            for j in 1..size {
                line.push(self.image[i][j]);
            }
            result.push(line);
        }

        result
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
    let mut tiles = prepare_input(fs::read_to_string(Path::new("./data/day20.txt"))?);
    let result;

    let mut image: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut placed_tiles: HashMap<i64, (isize, isize)> = HashMap::new();

    let mut min_coordinates = (0isize, 0isize);
    let mut max_coordinates = (0isize, 0isize);

    {
        let first_tile = tiles[0].clone();
        placed_tiles.insert(first_tile.id, (0, 0));
        image.insert((0, 0), first_tile);
    }

    while placed_tiles.len() != tiles.len() {
        for tile in tiles.iter_mut() {
            if placed_tiles.contains_key(&tile.id) {
                continue;
            }

            let mut tile_to_insert = None;
            let mut coordinate_to_insert = None;

            for (coordinate, placed_tile) in image.iter() {
                if let Some(side) = tile.fit_to_other(&placed_tile) {
                    let tile_to_be_placed = tile.clone();
                    let mut next_coordinate = *coordinate;
                    match side {
                        Top => {
                            next_coordinate.1 += 1;
                        }
                        Right => {
                            next_coordinate.0 -= 1;
                        }
                        Bottom => {
                            next_coordinate.1 -= 1;
                        }
                        Left => {
                            next_coordinate.0 += 1;
                        }
                    }
                    tile_to_insert = Some(tile_to_be_placed.clone());
                    coordinate_to_insert = Some(next_coordinate);
                    placed_tiles.insert(tile_to_be_placed.id, next_coordinate);
                }
            }
            if let Some(coord) = coordinate_to_insert {
                min_coordinates.0 = std::cmp::min(min_coordinates.0, coord.0);
                min_coordinates.1 = std::cmp::min(min_coordinates.1, coord.1);
                max_coordinates.0 = std::cmp::max(max_coordinates.0, coord.0);
                max_coordinates.1 = std::cmp::max(max_coordinates.1, coord.1);

                let insert_tile = tile_to_insert.unwrap();
                image.insert(coord, insert_tile);
            }
        }
    }

    result = {
        image.get(&min_coordinates).unwrap().id
            * image.get(&max_coordinates).unwrap().id
            * image
                .get(&(min_coordinates.0, max_coordinates.1))
                .unwrap()
                .id
            * image
                .get(&(max_coordinates.0, min_coordinates.1))
                .unwrap()
                .id
    };

    println!("Result from corner images id: {}", result);

    Ok(())
}

fn rotate_image(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_image = vec![];
    let size = image.len();

    for col in 0..size {
        let mut line = vec![];
        for row in (0..size).rev() {
            line.push(image[row][col]);
        }
        new_image.push(line);
    }

    new_image
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut tiles = prepare_input(fs::read_to_string(Path::new("./data/day20.txt"))?);

    let mut image: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut placed_tiles: HashMap<i64, (isize, isize)> = HashMap::new();

    let mut min_coordinates = (0isize, 0isize);
    let mut max_coordinates = (0isize, 0isize);

    {
        let first_tile = tiles[0].clone();
        placed_tiles.insert(first_tile.id, (0, 0));
        image.insert((0, 0), first_tile);
    }

    while placed_tiles.len() != tiles.len() {
        for tile in tiles.iter_mut() {
            if placed_tiles.contains_key(&tile.id) {
                continue;
            }

            let mut tile_to_insert = None;
            let mut coordinate_to_insert = None;

            for (coordinate, placed_tile) in image.iter() {
                if let Some(side) = tile.fit_to_other(&placed_tile) {
                    let tile_to_be_placed = tile.clone();
                    let mut next_coordinate = *coordinate;
                    match side {
                        Top => {
                            next_coordinate.1 += 1;
                        }
                        Right => {
                            next_coordinate.0 -= 1;
                        }
                        Bottom => {
                            next_coordinate.1 -= 1;
                        }
                        Left => {
                            next_coordinate.0 += 1;
                        }
                    }
                    tile_to_insert = Some(tile_to_be_placed.clone());
                    coordinate_to_insert = Some(next_coordinate);
                    placed_tiles.insert(tile_to_be_placed.id, next_coordinate);
                }
            }
            if let Some(coord) = coordinate_to_insert {
                min_coordinates.0 = std::cmp::min(min_coordinates.0, coord.0);
                min_coordinates.1 = std::cmp::min(min_coordinates.1, coord.1);
                max_coordinates.0 = std::cmp::max(max_coordinates.0, coord.0);
                max_coordinates.1 = std::cmp::max(max_coordinates.1, coord.1);

                let insert_tile = tile_to_insert.unwrap();
                image.insert(coord, insert_tile);
            }
        }
    }

    let mut full_picture = vec![];

    for j in min_coordinates.1..=max_coordinates.1 {
        let mut lines_tile = vec![];
        for i in min_coordinates.0..=max_coordinates.0 {
            let tile = image.get(&(i, j)).unwrap().get_borderless_picture();
            if lines_tile.is_empty() {
                lines_tile = tile.clone();
            } else {
                for (i, line) in tile.iter().enumerate() {
                    lines_tile[i].append(&mut line.clone());
                }
            }
        }
        full_picture.append(&mut lines_tile);
    }

    let dragon_scales_coordinates = [
        (0, 1),
        (1, 2),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        (16, 2),
        (17, 1),
        (18, 0),
        (18, 1),
        (19, 1),
    ];
    let mut dragons_found = 0;

    let size = full_picture.len();

    let max_roughness = full_picture.iter().fold(0, |acc, line| {
        acc + line.iter().fold(
            0,
            |acc, character| if *character == '#' { acc + 1 } else { acc },
        )
    });

    for count in 0..8 {
        for line_no in 0..size {
            for col_no in 0..size {
                let mut is_dragon = true;
                for (offset_x, offset_y) in dragon_scales_coordinates.iter() {
                    let x = offset_x + col_no;
                    let y = offset_y + line_no;

                    if let Some(line) = full_picture.get(y) {
                        if let Some(value) = line.get(x) {
                            if let '.' = value {
                                is_dragon = false;
                                break;
                            }
                        } else {
                            is_dragon = false;
                            break;
                        }
                    } else {
                        is_dragon = false;
                        break;
                    }
                }
                if is_dragon {
                    dragons_found += 1;
                    for (offset_x, offset_y) in dragon_scales_coordinates.iter() {
                        let x = offset_x + col_no;
                        let y = offset_y + line_no;
                        full_picture[y][x] = 'O';
                    }
                }
            }
        }
        if dragons_found > 0 {
            break;
        }
        full_picture = rotate_image(full_picture);
        if count == 3 {
            full_picture = full_picture.iter().cloned().rev().collect();
        }
    }

    for line in full_picture.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    println!(
        "The habitat's water roughness is {}",
        max_roughness - dragons_found * 15
    );

    Ok(())
}
