use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::VecDeque;

use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher};

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

// Took it from https://github.com/jlricon/advent-2020/blob/master/src/bin/day22.rs
// Actually order of magnitude faster than using an hashset of strings like I did before.
fn hash_game(deck_1: &VecDeque<usize>, deck_2: &VecDeque<usize>) -> u64 {
    let mut hash = DefaultHasher::new();
    let mut hash2 = DefaultHasher::new();
    deck_1.hash(&mut hash);
    deck_2.hash(&mut hash2);
    hash.finish() * hash2.finish()
}

fn recursive_combat(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> (bool, usize) {
    use std::collections::HashSet;

    let mut is_player_1_winning = false;
    let score;

    let mut infinite_loop_guard: HashSet<u64> = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        let signature = hash_game(&player_1, &player_2);

        if !infinite_loop_guard.insert(signature) {
            return (
                true,
                player_1
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (index, value)| acc + (index + 1) * value),
            );
        }

        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();

        let (target_deck, winning_card, losing_card) =
            if card_1 <= player_1.len() && card_2 <= player_2.len() {
                let sub_deck_1 = player_1
                    .iter()
                    .cloned()
                    .take(card_1)
                    .collect::<VecDeque<_>>();
                let sub_deck_2 = player_2
                    .iter()
                    .cloned()
                    .take(card_2)
                    .collect::<VecDeque<_>>();

                let (result, _) = recursive_combat(sub_deck_1, sub_deck_2);
                if result {
                    (&mut player_1, card_1, card_2)
                } else {
                    (&mut player_2, card_2, card_1)
                }
            } else if card_1 > card_2 {
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
        is_player_1_winning = true;
        &mut player_1
    };

    score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index + 1) * value);

    (is_player_1_winning, score)
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let decks = prepare_input(fs::read_to_string(Path::new("data/day22.txt"))?);
    let player_1 = decks[0].clone();
    let player_2 = decks[1].clone();

    let (result, score) = recursive_combat(player_1, player_2);

    println!("Is player 1 winning? {} With score {}", result, score);

    Ok(())
}
// 33886 too low
