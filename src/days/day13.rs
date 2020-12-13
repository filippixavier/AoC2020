use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.trim().lines();
    let timestamp = lines.next().unwrap().parse::<usize>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|bus| bus.parse::<usize>().ok())
        .collect();

    (timestamp, buses)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (timestamp, buses) = prepare_input(fs::read_to_string(Path::new("data/day13.txt"))?);

    let next_arrival = buses
        .iter()
        .filter(|bus| bus.is_some())
        .map(|bus| {
            let bus_id = bus.unwrap();
            let bus_cycles = timestamp / bus_id;
            if bus_cycles * bus_id < timestamp {
                (bus_id, bus_id * (bus_cycles + 1))
            } else {
                (bus_id, timestamp)
            }
        })
        .min_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b))
        .unwrap();

    println!(
        "The earliest bus is ID-{} comming at timestamp {}, resulting it in {}",
        next_arrival.0,
        next_arrival.1,
        (next_arrival.1 - timestamp) * next_arrival.0
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
