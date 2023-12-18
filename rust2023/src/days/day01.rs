use crate::common::get_lines;
use crate::Args;

pub fn run(_args: &Args) -> (String, String) {
    let lines = get_lines("day01");

    let part_1_value = lines
        .iter()
        .map(|x| get_number_occurrences(x, digit_word_list()))
        .map(concat_first_and_last)
        .sum::<isize>();

    let part_2_value = lines
        .iter()
        .map(|x| get_number_occurrences(x, big_word_list()))
        .map(concat_first_and_last)
        .sum::<isize>();

    (part_1_value.to_string(), part_2_value.to_string())
}

fn big_word_list() -> Vec<(String, u8)> {
    vec![
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]
}
fn digit_word_list() -> Vec<(String, u8)> {
    vec![
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
    ]
}

fn concat_first_and_last(number_occurs: Vec<(usize, u8)>) -> isize {
    let first = number_occurs
        .iter()
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .unwrap()
        .1;

    let last = number_occurs
        .iter()
        .reduce(|a, b| if a.0 > b.0 { a } else { b })
        .unwrap()
        .1;

    format!("{first}{last}").parse::<isize>().unwrap()
}

fn get_number_occurrences(line: &String, words: Vec<(String, u8)>) -> Vec<(usize, u8)> {
    let mut all_occurs = Vec::new();

    for (word, value) in words {
        let mut occurrences: Vec<_> = line
            .match_indices(&word)
            .map(|(index, _)| (index, value))
            .collect();

        all_occurs.append(&mut occurrences);
    }

    all_occurs
}
