use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::VecDeque;

fn prepare_input(input: String) -> VecDeque<u32> {
    input
        .split('\n')
        .map(|x| (x.trim().parse::<u32>().unwrap_or(0)))
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut arr: VecDeque<u32> = prepare_input(fs::read_to_string(Path::new("./data/day1.txt"))?);
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
    let arr: VecDeque<u32> = prepare_input(fs::read_to_string(Path::new("./data/day1.txt"))?);
    for (index, first_num) in arr.iter().enumerate() {
        for (index_2, second_num) in arr.iter().enumerate().skip(index) {
            for third_num in arr.iter().skip(index_2) {
                if first_num + second_num + third_num == 2020 {
                    println!("Result 2 is: {}", first_num * second_num * third_num);
                    return Ok(())
                }
            }
        }
    }
    Ok(())
}
