use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::VecDeque;

fn prepare_input(input: String) -> Vec<VecDeque<usize>> {
    let mut result = vec![];

    let players = input.trim().split("\r\n\r\n").collect::<Vec<_>>();

    for player in players {
        let cards = player
            .lines()
            .skip(1)
            .map(str::parse)
            .collect::<Result<VecDeque<usize>, _>>()
            .unwrap();
        result.push(cards);
    }

    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let decks = prepare_input(fs::read_to_string(Path::new("data/day22.txt"))?);
    let mut player_1 = decks[0].clone();
    let mut player_2 = decks[1].clone();

    while !player_1.is_empty() && !player_2.is_empty() {
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();

        let (target_deck, winning_card, losing_card) = if card_1 > card_2 {
            (&mut player_1, card_1, card_2)
        } else {
            (&mut player_2, card_2, card_1)
        };

        target_deck.push_back(winning_card);
        target_deck.push_back(losing_card);
    }

    let winning_deck = if player_1.is_empty() {
        &mut player_2
    } else {
        &mut player_1
    };

    let score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index + 1) * value);

    println!("Winning player score is: {}", score);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
