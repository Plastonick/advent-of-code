use crate::common::get_lines;
use crate::Args;
use std::collections::HashMap;
use std::fmt::format;

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day07-test")
    } else {
        get_lines("day07")
    };

    let cards = lines
        .iter()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(cards, rank)| {
            (
                cards.chars().collect::<Vec<_>>(),
                rank.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut ranked_card = cards
        .iter()
        .map(|(hand, bid)| (score_as_joker(&hand), hand, bid))
        .collect::<Vec<_>>();

    ranked_card.sort_by(|(a_rank, _, _), (b_rank, _, _)| a_rank.cmp(b_rank));

    let sorted = ranked_card
        .iter()
        .map(|(x, y, z)| format!("{} - {}", y.iter().collect::<String>(), x))
        .collect::<Vec<_>>();

    dbg!(sorted);

    let part_1_score: usize = ranked_card
        .iter()
        .enumerate()
        .map(|(rank, (_, _, &bid))| (rank + 1) * bid)
        .sum();

    if !args.no_answers {
        println!("Day 7, Part 1: {part_1_score}");
        println!("Day 7, Part 2: TODO");
    }

    ("".to_string(), "".to_string())
}

fn score(cards: &Vec<char>) -> usize {
    let trick: usize = count_types(&cards).values().map(|v| v * v).sum();

    let best_card_value: usize = cards
        .iter()
        .enumerate()
        .map(|(i, r)| 15_usize.pow((4 - i) as u32) * card_val(&r))
        .sum();

    trick * 15_usize.pow(5) + best_card_value
}

fn count_types(cards: &Vec<char>) -> HashMap<&char, usize> {
    cards.iter().fold(HashMap::new(), |mut acc, el| {
        *acc.entry(el).or_insert(0) += 1;
        acc
    })
}

fn score_as_joker(cards: &Vec<char>) -> usize {
    let mut card_counts = count_types(&cards);
    let best_non_joker = card_counts
        .iter()
        .filter(|(&card, _)| card != &'J')
        .reduce(|a, b| if a.1 > b.1 { a } else { b });

    if let Some(card) = best_non_joker {
        if let Some(joker_count) = card_counts.get(&'J') {
            *card_counts.entry(card.0).or_insert(0) += joker_count.clone();
        }
    }

    card_counts.remove(&'J');

    let trick: usize = card_counts.values().map(|v| v * v).sum();

    let best_card_value: usize = cards
        .iter()
        .enumerate()
        .map(|(i, r)| 15_usize.pow((4 - i) as u32) * card_val_2(&r))
        .sum();

    trick * 15_usize.pow(5) + best_card_value
}

fn card_val(card: &char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => (*card as usize) - ('0' as usize),
    }
}

fn card_val_2(card: &char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => (*card as usize) - ('0' as usize),
    }
}
