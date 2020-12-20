use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use regex::Regex;

fn is_recurse(identifier: usize, code: &str) -> bool {
    for element in code.trim().split(' ') {
        if let Ok(value) = element.parse::<usize>() {
            if value == identifier {
                return true;
            }
        }
    }
    false
}

// Just because I could use a Regex.
fn recurse_get_rule(
    identifier: usize,
    rule_set: &HashMap<usize, String>,
) -> (String, Vec<Option<String>>) {
    let mut resulting_rule = String::from("");
    let current_rule = rule_set.get(&identifier).unwrap();
    let have_splitter = current_rule.contains('|');
    let mut special_cases = vec![];

    if identifier == 0 {
        resulting_rule += "^";
    }

    let recursive = is_recurse(identifier, current_rule);

    if recursive {
        let sub_rule = current_rule.split(" | ").next().unwrap();
        for i in sub_rule.trim().split(' ') {
            let mut sub_str = String::from("");
            match i {
                "|" => sub_str += "|",
                "\"a\"" | "\"b\"" => return (i.chars().nth(1).unwrap().to_string(), special_cases),
                _ => {
                    if let Ok(id) = i.parse::<usize>() {
                        let (recursed_rule, _) = recurse_get_rule(id, rule_set);
                        sub_str += recursed_rule.as_str();
                    }
                }
            }
            if identifier == 11 {
                special_cases.push(Some(sub_str.clone()));
            }
            resulting_rule += format!("(:?{})+", sub_str).as_str();
        }
    } else {
        for i in current_rule.trim().split(' ') {
            match i {
                "|" => resulting_rule += "|",
                "\"a\"" | "\"b\"" => return (i.chars().nth(1).unwrap().to_string(), special_cases),
                _ => {
                    if let Ok(id) = i.parse::<usize>() {
                        let (recursed_rule, spec) = recurse_get_rule(id, rule_set);
                        special_cases = spec;
                        resulting_rule += recursed_rule.as_str();
                    }
                }
            }
        }
        if have_splitter {
            resulting_rule = format!("(?:{})", resulting_rule);
        }
    }

    if identifier == 0 {
        resulting_rule += "$";
    }

    (resulting_rule, special_cases)
}

fn prepare_input(input: String) -> (HashMap<usize, String>, Vec<String>) {
    let parts = input.trim().split("\r\n\r\n").collect::<Vec<_>>();

    let rule_set: HashMap<usize, String> = parts[0]
        .trim()
        .lines()
        .map(|line| {
            let tmp = line.split(':').collect::<Vec<_>>();
            let index = tmp[0].parse::<usize>().unwrap();
            (index, tmp[1].to_string())
        })
        .collect();

    let messages = parts[1]
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    (rule_set, messages)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (rule_set, msgs) = prepare_input(fs::read_to_string(Path::new("./data/day19.txt"))?);

    let mut count = 0;
    let (final_rule, _) = recurse_get_rule(0, &rule_set);
    let reg = Regex::new(final_rule.as_str()).unwrap();

    for msg in msgs {
        if reg.is_match(msg.as_str()) {
            count += 1;
        }
    }

    println!("Number of right inputs: {}", count);

    Ok(())
}

// Other possible answer include using a CYK algorithm, which whould have been easier...and probably faster
// https://en.wikipedia.org/wiki/CYK_algorithm
fn prepare_input_2(input: String) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut rules = HashMap::new();
    let msgs;

    let blocs = input.trim().split("\r\n\r\n").collect::<Vec<_>>();
    msgs = blocs[1]
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    for line in blocs[0].lines() {
        let name_value = line.split(':').collect::<Vec<_>>();
        let name = name_value[0];
        let value = name_value[1].trim();

        let rule_set = value
            .split(" | ")
            .map(|element| element.trim().replace('"', ""))
            .collect::<Vec<_>>();

        rules.insert(name.to_string(), rule_set);
    }

    (rules, msgs)
}

fn is_match(
    msg: &[char],
    positions: &[usize],
    rule_id: String,
    rule_set: &HashMap<String, Vec<String>>,
) -> (bool, Vec<usize>) {
    let rules = rule_set.get(&rule_id).unwrap();
    let mut is_valid = false;

    let starting_pos = positions
        .iter()
        .cloned()
        .filter(|&pos| pos < msg.len())
        .collect::<Vec<usize>>();

    let mut next_positions = starting_pos.clone();

    let mut ending_pos = vec![];

    if starting_pos.is_empty() {
        return (false, vec![]);
    }

    for rule in rules.iter() {
        let ids = rule.split(' ');
        for id in ids {
            match id {
                // Always return since terminal rules are always alone
                "a" | "b" => {
                    for posi in next_positions.iter() {
                        if id.chars().next().unwrap() == msg[*posi] {
                            ending_pos.push(posi + 1);
                        }
                    }
                    if ending_pos.is_empty() {
                        return (false, vec![]);
                    }
                    return (true, ending_pos);
                }
                _ => {
                    let mut intermediate = vec![];
                    for pos in next_positions.iter() {
                        let (is_sub_valid, mut valid_sub_pos) =
                            is_match(msg, &[*pos], id.to_string(), rule_set);
                        if is_sub_valid {
                            intermediate.append(&mut valid_sub_pos);
                        }
                    }

                    is_valid = !intermediate.is_empty();
                    if is_valid {
                        next_positions = intermediate;
                    } else {
                        next_positions = starting_pos.clone();
                        break;
                    }
                }
            }
        }

        if is_valid {
            is_valid = false;
            ending_pos.append(&mut next_positions);
            next_positions = starting_pos.clone();
        }
    }

    ending_pos.sort_unstable();
    ending_pos.dedup();
    (!ending_pos.is_empty(), ending_pos)
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut rule_set, msgs) = prepare_input_2(fs::read_to_string(Path::new("./data/day19.txt"))?);
    let mut count = 0;
    rule_set.insert("8".to_string(), vec!["42".to_string(), "42 8".to_string()]);
    rule_set.insert(
        "11".to_string(),
        vec!["42 31".to_string(), "42 11 31".to_string()],
    );

    for msg in msgs {
        let msg_chars = msg.chars().collect::<Vec<_>>();
        let (result, final_pos) = is_match(&msg_chars, &[0], "0".to_string(), &rule_set);
        if result && final_pos.contains(&msg.len()) {
            count += 1;
        }
    }

    println!("After fixing, number of right inputs: {}", count);

    Ok(())
}
