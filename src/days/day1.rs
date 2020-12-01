use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<u32> {
    input
        .split('\n')
        .map(|x| (x.trim().parse::<u32>().unwrap_or(0)))
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let arr = prepare_input(fs::read_to_string(Path::new("./data/day1.txt"))?);
    for (index, first_num) in arr.iter().enumerate() {
        for second_num in arr.iter().skip(index) {
            if first_num + second_num == 2020 {
                println!("Result is: {}", first_num * second_num);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let arr = prepare_input(fs::read_to_string(Path::new("./data/day1.txt"))?);
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
