use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::VecDeque;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut arr: VecDeque<u32> = fs::read_to_string(Path::new("./data/day1.txt"))?
        .split('\n')
        .map(|x| (x.trim().parse::<u32>().unwrap_or(0)))
        .collect();

    while !arr.is_empty() {
        let current = arr.pop_front().unwrap_or(0);
        for elem in arr.iter() {
            if elem + current == 2020 {
                println!("Result is: {}", elem * current);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!()
}
