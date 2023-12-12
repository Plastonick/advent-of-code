use crate::common::get_lines;
use crate::Args;
use std::collections::HashMap;

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
        .map(|(hand, bid)| (score(&hand), hand, bid))
        .collect::<Vec<_>>();

    ranked_card.sort_by(|(a_rank, _, _), (b_rank, _, _)| a_rank.cmp(b_rank));

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
    let trick: usize = cards
        .iter()
        .fold(HashMap::new(), |mut acc, el| {
            *acc.entry(el).or_insert(0) += 1;
            acc
        })
        .values()
        .map(|v| v * v) // trick to prefer larger sets
        .sum();

    let best_card_value: usize = cards
        .iter()
        .enumerate()
        .map(|(i, r)| 15_usize.pow((4 - i) as u32) * card_val(&r))
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
