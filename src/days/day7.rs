use regex::Regex;
use std::collections::*;
use std::error::Error;
use std::fs;
use std::path::Path;

type Bags = HashMap<String, HashMap<String, usize>>;

fn prepare_input(input: String) -> Bags {
    let holded_bags_reg = Regex::new(r"(\d) ((?:\w|\s)+) bag").unwrap();
    let mut bags = Bags::new();

    for rule in input.trim().lines() {
        let mut holded_bags = HashMap::new();

        let mut tmp = rule.split(" contain ");
        let holder = tmp.next().unwrap();
        let holder = holder.split(" bags").next().unwrap();
        let holdee = tmp.next().unwrap();

        for bags in holdee.split(',') {
            if let Some(cap) = holded_bags_reg.captures(bags) {
                holded_bags.insert(cap[2].to_owned(), cap[1].parse().unwrap());
            }
        }

        bags.insert(holder.to_owned(), holded_bags);
    }

    bags
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let bags = prepare_input(fs::read_to_string(Path::new("./data/day7.txt"))?);
    let mut recorded_bags: HashSet<String> = HashSet::new();
    let mut bag_to_check = vec![String::from("shiny gold")];

    while !bag_to_check.is_empty() {
        let goal = bag_to_check.pop().unwrap();

        if recorded_bags.contains(&goal) {
            continue;
        }

        recorded_bags.insert(goal.clone());

        for (holder, holdee) in bags.iter() {
            if holdee.contains_key(&goal) {
                bag_to_check.push(holder.clone());
            }
        }
    }

    println!(
        "I can fit my beautiful shiny gold bag into {} bags",
        recorded_bags.len() - 1
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
