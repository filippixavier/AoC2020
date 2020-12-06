use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::iter::FromIterator;
    let input = fs::read_to_string(Path::new("./data/day6.txt"))?;
    let all_answers = input.trim().split("\r\n\r\n").map(|group| {
        let mut answers = String::from(group);
        answers.retain(|c| !c.is_whitespace());
        HashSet::<char>::from_iter(answers.chars())
    });

    let value = all_answers.fold(0, |acc, group_answers| acc + group_answers.len());

    println!(
        "The sum of yes answered question accross every groups is {}",
        value
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
