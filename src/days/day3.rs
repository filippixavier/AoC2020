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
        if let Tree = line.iter().cycle().nth(left_coordinate).unwrap() {
            tree_count += 1
        }
        left_coordinate += 3;
    }

    println!("You would encounter {} trees on your way down", tree_count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use Tile::*;
    let forest = prepare_input(fs::read_to_string(Path::new("./data/day3.txt"))?);
    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let line_len = forest[0].len();

    let mut total_trees: i64 = 1;

    for (step_right, step_down) in &steps {
        let mut coordinates = (0, 0);
        let mut trees = 0;

        while coordinates.0 < forest.len() {
            if let Tree = forest[coordinates.0][coordinates.1] {
                trees += 1
            }
            coordinates.0 += step_down;
            coordinates.1 = (coordinates.1 + step_right) % line_len;
        }
        total_trees *= trees;
    }

    println!(
        "Using each pattern, the tree score would be {}",
        total_trees
    );

    Ok(())
}
