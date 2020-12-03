use std::error::Error;
use std::fs;
use std::path::Path;

enum Tile {
    Tree,
    Empty,
}

fn prepare_input(input: String) -> Vec<Vec<Tile>> {
    let mut forest = vec![];

    for line in input.trim().lines() {
        let mut forest_line = vec![];
        for tile in line.trim().chars() {
            match tile {
                '.' => forest_line.push(Tile::Empty),
                '#' => forest_line.push(Tile::Tree),
                _ => unreachable!(),
            }
        }
        forest.push(forest_line);
    }

    forest
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use Tile::*;

    let forest = prepare_input(fs::read_to_string(Path::new("./data/day3.txt"))?);
    let mut left_coordinate = 0;
    let mut tree_count = 0;

    for line in forest.iter() {
        match line.iter().cycle().nth(left_coordinate).unwrap() {
            Tree => tree_count += 1,
            Empty => {}
        }
        left_coordinate += 3;
    }

    println!("You would encounter {} trees on your way down", tree_count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
