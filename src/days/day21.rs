use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

fn prepare_input(input: String) -> Vec<(Vec<String>, Vec<String>)> {
    let reg = Regex::new(r"((?:\w+ ?)+) \(contains ((?:\w+(?:, )?)+)").unwrap();

    let mut result = vec![];

    for cap in reg.captures_iter(&input) {
        let ingredients = cap[1].split(' ').map(|elem| elem.to_string()).collect();
        let allergens = cap[2].split(", ").map(|elem| elem.to_string()).collect();
        result.push((ingredients, allergens));
    }

    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let ingredients_list = prepare_input(fs::read_to_string(Path::new("./data/day21.txt"))?);
    // let non_allergics_ingredients: HashSet<String>= HashSet::new();
    let mut possible_ingredients_per_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for (ingredients, allergens) in ingredients_list.iter() {
        let list = ingredients.iter().cloned().collect::<HashSet<_>>();
        for allergen in allergens.iter() {
            let possible_ingredient =
                if let Some(possible) = possible_ingredients_per_allergens.get(allergen) {
                    possible
                } else {
                    &list
                };

            let intersection = possible_ingredient
                .intersection(&list)
                .collect::<HashSet<_>>();
            let intersection = intersection
                .iter()
                .map(|&i| i.clone())
                .collect::<HashSet<_>>();

            possible_ingredients_per_allergens.insert(allergen.clone(), intersection);
        }
    }

    let mut killer_food = HashSet::<String>::new();

    for ingredients_list in possible_ingredients_per_allergens.values() {
        let tmp = killer_food.union(&ingredients_list);
        killer_food = tmp.into_iter().cloned().collect();
    }

    let mut count = 0;

    for (ingredients, _) in ingredients_list.iter() {
        count += ingredients.len();
        for ingredient in ingredients.iter() {
            if killer_food.contains(ingredient) {
                count -= 1;
            }
        }
    }

    println!("Amount of non killing stuff: {}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
