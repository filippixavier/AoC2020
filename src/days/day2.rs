use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

struct PasswordCheck {
    pwd: String,
    rule: String,
    range: [usize; 2],
}

fn prepare_input(input: String) -> Vec<PasswordCheck> {
    let reg: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let lines = input.lines();
    let mut range: Vec<PasswordCheck> = vec![];
    for line in lines {
        for cap in reg.captures_iter(line) {
            let pwd_check = PasswordCheck {
                pwd: cap[4].to_string(),
                rule: cap[3].to_string(),
                range: [
                    cap[1].to_string().parse().unwrap(),
                    cap[2].to_string().parse().unwrap(),
                ],
            };
            range.push(pwd_check);
        }
    }
    range
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let inputs = prepare_input(fs::read_to_string(Path::new("./data/day2.txt"))?);
    let result = inputs
        .iter()
        .filter(|checker| {
            let value = &checker.pwd.matches(&checker.rule).count();
            value >= &checker.range[0] && value <= &checker.range[1]
        })
        .count();
    println!("There is {} valid passwords in input", result);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let inputs = prepare_input(fs::read_to_string(Path::new("./data/day2.txt"))?);
    let result = inputs
        .iter()
        .filter(|checker| {
            let rule = &checker.rule.chars().next().unwrap();
            let pwd = &checker.pwd.chars().collect::<Vec<char>>();
            (&pwd[checker.range[0] - 1] == rule) ^ (&pwd[checker.range[1] - 1] == rule)
        })
        .count();
    println!(
        "With the correct rule, there is {} valid passwords in input",
        result
    );
    Ok(())
}
