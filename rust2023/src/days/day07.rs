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
        .collect::<Vec<(Vec<char>, usize)>>();

    let part_1_winnings = get_winnings(&cards, score_regular);
    let part_2_winnings = get_winnings(&cards, score_with_joker);

    if !args.no_answers {
        println!("Day 7, Part 1: {part_1_winnings}");
        println!("Day 7, Part 2: {part_2_winnings}");
    }

    (part_1_winnings.to_string(), part_2_winnings.to_string())
}

fn get_winnings(
    cards: &Vec<(Vec<char>, usize)>,
    scoring_strategy: fn(&Vec<char>) -> usize,
) -> usize {
    let mut ranked_card = cards
        .iter()
        .map(|(hand, bid)| (scoring_strategy(&hand), hand, bid))
        .collect::<Vec<_>>();

    ranked_card.sort_by(|(a_rank, _, _), (b_rank, _, _)| a_rank.cmp(b_rank));

    ranked_card
        .iter()
        .enumerate()
        .map(|(rank, (_, _, &bid))| (rank + 1) * bid)
        .sum()
}

fn score_regular(cards: &Vec<char>) -> usize {
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

fn score_with_joker(cards: &Vec<char>) -> usize {
    let mut card_counts = count_types(&cards);
    let best_non_joker = card_counts
        .iter()
        .filter(|(&card, _)| card != &'J')
        .reduce(|a, b| if a.1 > b.1 { a } else { b })
        .unwrap_or((&&'A', &0))
        .0
        .to_owned();

    if let Some(joker_count) = card_counts.get(&'J') {
        *card_counts.entry(best_non_joker).or_insert(0) += joker_count.clone();
    }

    card_counts.remove(&'J');

    let trick: usize = card_counts.values().map(|v| v * v).sum();

    // essentially represents the cards as a base-15 number, which can then be directly compared
    let best_card_ranking_value: usize = cards
        .iter()
        .enumerate()
        .map(|(i, r)| 15_usize.pow((4 - i) as u32) * card_val_2(&r))
        .sum();

    // the "trick" i.e. pair, two pair, etc. is the highest order digit.
    trick * 15_usize.pow(5) + best_card_ranking_value
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
