use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let pks = prepare_input(fs::read_to_string(Path::new("./data/day25.txt"))?);
    let mut subject_number: u64 = 7;
    let mut encryption_key = 1u64;

    let mut loops_amount = 0;

    while !pks.contains(&encryption_key) {
        encryption_key = (encryption_key * subject_number) % 20201227;
        loops_amount += 1;
    }

    subject_number = if pks[0] == encryption_key {
        pks[1]
    } else {
        encryption_key
    };

    encryption_key = 1;

    for _ in 0..loops_amount {
        encryption_key = (encryption_key * subject_number) % 20201227;
    }

    println!(
        "The encryption key for the handshake should be: {}",
        encryption_key
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("No rest for the braves...");
    Ok(())
}
