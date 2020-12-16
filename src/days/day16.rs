use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

type RangeRules = (
    String,
    std::ops::RangeInclusive<u32>,
    std::ops::RangeInclusive<u32>,
);

fn prepare_input(input: String) -> (Vec<RangeRules>, Vec<u32>, Vec<Vec<u32>>) {
    let ticket_rules = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let blocs = input.split("\r\n\r\n").collect::<Vec<_>>();
    let input_rules = blocs[0];

    let mut rules: Vec<RangeRules> = vec![];

    for cap_ranges in ticket_rules.captures_iter(input_rules) {
        let mut iter_cap = cap_ranges.iter().skip(1);
        let title = String::from(iter_cap.next().unwrap().unwrap().as_str());
        let cap_ranges = iter_cap
            .map(|x| x.unwrap().as_str().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        rules.push((
            title,
            cap_ranges[0]..=cap_ranges[1],
            cap_ranges[2]..=cap_ranges[3],
        ));
    }

    let my_ticket = blocs[1]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()
                .unwrap()
        })
        .flatten()
        .collect::<Vec<_>>();

    let other_tickets = blocs[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    (rules, my_ticket, other_tickets)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (rules, _, other_tickets) = prepare_input(fs::read_to_string(Path::new("data/day16.txt"))?);

    let code: u32 = other_tickets
        .iter()
        .map(|ticket| {
            'outer: for value in ticket.iter() {
                for range in rules.iter() {
                    if range.1.contains(value) || range.2.contains(value) {
                        continue 'outer;
                    }
                }
                return *value;
            }
            0
        })
        .sum();

    println!("Ticket error scanning rate is: {}", code);

    Ok(())
}

use std::collections::HashMap;
use std::collections::HashSet;

fn deduce_positions(
    mut fields_possible_position: HashMap<usize, Option<HashSet<String>>>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    let mut continue_loop = true;

    while continue_loop {
        continue_loop = false;
        let mut to_remove = vec![];
        for (position, possible_fields) in fields_possible_position.iter() {
            if let Some(fields_names) = possible_fields {
                if fields_names.len() == 1 {
                    let valided_field = fields_names.iter().next().unwrap();
                    if !result.contains_key(valided_field) {
                        result.insert(valided_field.clone(), *position);
                        continue_loop = true;
                        to_remove.push((*position, valided_field.clone()));
                    }
                }
            }
        }

        for (position, name) in to_remove {
            for (index, names) in fields_possible_position.iter_mut() {
                if *index == position {
                    continue;
                }
                if let Some(fields) = names.as_mut() {
                    fields.remove(&name);
                }
            }
        }
    }

    result
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (rules, my_ticket, other_tickets) =
        prepare_input(fs::read_to_string(Path::new("data/day16.txt"))?);

    let mut fields_position: HashMap<usize, Option<HashSet<String>>> = HashMap::new();

    for i in 0..rules.len() {
        fields_position.insert(i, None);
    }

    for ticket in other_tickets {
        let mut has_invalid_field = false;
        let mut fields_possible_value: Vec<(usize, HashSet<String>)> = vec![];
        for (index, value) in ticket.iter().enumerate() {
            let mut is_valid = false;
            let mut field_names = HashSet::new();
            for (range_name, first_range, second_range) in rules.iter() {
                if first_range.contains(value) || second_range.contains(value) {
                    is_valid |= true;
                    field_names.insert(range_name.clone());
                }
            }
            if is_valid {
                fields_possible_value.push((index, field_names));
            }
            has_invalid_field = !is_valid;
        }
        if !has_invalid_field {
            for (index, possible_values) in fields_possible_value {
                if let Some(fields_names_candidate) = fields_position.get_mut(&index) {
                    if fields_names_candidate.is_none() {
                        *fields_names_candidate = Some(possible_values);
                    } else {
                        let mut new_fields = HashSet::new();
                        if let Some(stuff) = fields_names_candidate {
                            new_fields = stuff.intersection(&possible_values).cloned().collect();
                        }
                        *fields_names_candidate = Some(new_fields);
                    }
                }
            }
        }
    }

    let fields_position = deduce_positions(fields_position);

    let result = fields_position
        .into_iter()
        .filter(|(key, _)| key.contains("departure"))
        .fold(1i64, |acc, (_, pos)| acc * my_ticket[pos] as i64);

    println!("Departure code is: {}", result);

    Ok(())
}
