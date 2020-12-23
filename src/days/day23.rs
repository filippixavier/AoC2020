use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input(input: String) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|value| value.to_string().parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

// pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
//     let mut cups = prepare_input(fs::read_to_string(Path::new("./data/day23.txt"))?);

//     let len = cups.len();

//     for i in 0..10 {
//         println!("CUPS: {:?}", cups);

//         let index = i % len;
//         let mut target_id = cups[index];
//         let took_cup = cups.iter().cycle().skip(index + 1).take(3);

//         let first_part = cups.iter().take(index + 1);
//         let intermediate = first_part.chain(cups.iter().cycle().skip(index + 4).take(len.checked_sub(index + 4).unwrap_or(0))).cloned().collect::<Vec<_>>();
//         println!("|{}|", target_id);

//         let mut found_pos = None;

//         while found_pos.is_none() {
//             if target_id == 0 {
//                 target_id = 10;
//             }
//             target_id -= 1;
//             found_pos = intermediate.iter().position(|&x| x == target_id);
//         }

//         if let Some(position) = found_pos {
//             let first_part = intermediate.iter().take(position + 1);
//             let second_part = intermediate.iter().skip(position + 1);
//             cups = first_part.chain(took_cup).chain(second_part).cloned().collect();
//         }
//     }

//     let cup_one = cups.iter().position(|&x| x == 1).unwrap();

//     println!("{}", cups.iter().skip(cup_one).fold(1, |acc, &value| acc * value));

//     Ok(())
// }

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let cups = prepare_input(fs::read_to_string(Path::new("./data/day23.txt"))?);
    let mut cycled_cups: HashMap<usize, usize> = HashMap::new();

    for (index, &cup_no) in cups.iter().enumerate() {
        let cup_neighbor = cups[(index + 1) % cups.len()] - 1;
        cycled_cups.insert(cup_no - 1, cup_neighbor);
    }

    let mut current_cup = cups[0] - 1;

    for _ in 0..100 {
        let pick1 = *cycled_cups.get(&current_cup).unwrap();
        let pick2 = *cycled_cups.get(&pick1).unwrap();
        let pick3 = *cycled_cups.get(&pick2).unwrap();
        let next = *cycled_cups.get(&pick3).unwrap();

        cycled_cups.insert(current_cup, next);

        let mut destination = current_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        while destination == pick1 || destination == pick2 || destination == pick3 {
            destination = destination.checked_sub(1).unwrap_or(cups.len() - 1);
        }

        cycled_cups.insert(pick3, *cycled_cups.get(&destination).unwrap());
        cycled_cups.insert(destination, pick1);
        current_cup = next;
    }

    while current_cup != 0 {
        current_cup = *cycled_cups.get(&current_cup).unwrap();
    }

    let mut result = String::new();

    for _ in 0..cups.len() - 1 {
        current_cup = *cycled_cups.get(&current_cup).unwrap();
        result.push_str(&(current_cup + 1).to_string());
    }

    println!("Cup labels: {}", result);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
