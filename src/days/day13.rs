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

// Copied from https://www.csee.umbc.edu/~chang/cs203.s09/exteuclid.shtml, needed http://defeo.lu/in310/poly/euclide-bezout/ and https://www.mathraining.be/chapters/4?type=1&which=16 to understand it
// Inverse modulo is located in the 3 tuple
fn extended_euclid(max: i128, min: i128) -> (i128, i128, i128) {
    if min == 0 {
        (max, 1, 0)
    } else {
        let (d1, s1, t1) = extended_euclid(min, max % min);
        (d1, t1, s1 - (max / min) * t1) // See http://defeo.lu/in310/poly/euclide-bezout/ on recursive relation
    }
}

//http://www.bibmath.net/dico/index.php?action=affiche&quoi=./c/chinois.html
pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (_, buses) = prepare_input(fs::read_to_string(Path::new("data/day13.txt"))?);
    let valid_buses = buses
        .iter()
        .enumerate()
        .filter(|(_, bus)| bus.is_some())
        .map(|(i, bus)| (i as i128, bus.unwrap() as i128))
        .collect::<Vec<(i128, i128)>>();

    let total = valid_buses.iter().fold(1, |acc, (_, bus_id)| acc * bus_id);

    let chinese_remainder: i128 = valid_buses
        .iter()
        .map(|(index, bus_id)| {
            let sub_total = total / bus_id;
            let x = bus_id - index;
            let (_, _, mut reverse_sub) = extended_euclid(*bus_id, sub_total);
            if reverse_sub < 0 {
                reverse_sub += *bus_id;
            }
            x * sub_total * reverse_sub
        })
        .sum();

    println!("{}", chinese_remainder % total);
    Ok(())
}
