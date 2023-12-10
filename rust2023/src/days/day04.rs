use crate::common::get_lines;
use crate::Args;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    my_numbers: HashSet<usize>,
}

impl Card {
    fn get_value(&self) -> usize {
        let size = self.matching_numbers() as u32;

        if size == 0 {
            0
        } else {
            let base: usize = 2;

            base.pow(size - 1)
        }
    }

    fn matching_numbers(&self) -> usize {
        self.winning_numbers.intersection(&self.my_numbers).count()
    }
}

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day04-test")
    } else {
        get_lines("day04")
    };

    let cards = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (i + 1, to_card(l)))
        .collect::<HashMap<usize, Card>>();
    let part_1_value = cards.iter().map(|(_, c)| c.get_value()).sum::<usize>();

    let mut won_cards = cards
        .iter()
        .map(|(number, _)| (*number, 1))
        .collect::<HashMap<usize, usize>>();
    let mut card_index = 1;

    while let Some(try_card) = cards.get(&card_index) {
        let number_cards = won_cards.get(&card_index).unwrap().clone();
        let number_won = try_card.matching_numbers();

        for i in card_index + 1..=card_index + number_won {
            *won_cards.entry(i).or_insert(0) += number_cards;
        }

        card_index += 1;
    }

    let total_cards: usize = won_cards.values().sum();

    if !args.no_answers {
        println!("Day 4, Part 1: The card score sum is {part_1_value}");
        println!("Day 4, Part 2: The total number of won cards is {total_cards}");
    }

    (part_1_value.to_string(), "".to_string())
}

fn to_card(line: &String) -> Card {
    let (_, data) = line.split_once(':').unwrap();
    let (winning, my) = data.split_once('|').unwrap();
    let winning_numbers = winning
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<HashSet<_>>();
    let my_numbers = my
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<HashSet<_>>();

    Card {
        winning_numbers,
        my_numbers,
    }
}
