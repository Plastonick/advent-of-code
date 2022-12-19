use crate::{common::get_lines, Args};

pub fn run(args: &Args) {
    let lines = get_lines("day02");

    let part_1_score = lines
        .iter()
        .map(|x| score_part_1(x))
        .reduce(|x, y| x + y)
        .expect("Uh oh! Couldn't score part 1");

    let part_2_score = lines
        .iter()
        .map(|x| score_part_2(x))
        .reduce(|x, y| x + y)
        .expect("Uh oh! Couldn't score part 2");

    if !args.no_answers {
        println!("Day 2, Part 1: scores {part_1_score}");
        println!("Day 2, Part 2: scores {part_2_score}");
    }
}

fn score_part_1(line: &String) -> usize {
    let goes = line.split(' ').take(2).collect::<Vec<&str>>();
    let [opp, you] = <[&str; 2]>::try_from(goes).ok().unwrap();

    let opp_index = match opp {
        "A" => 0,
        "B" => 1,
        _ => 2,
    };

    let you_index = match you {
        "X" => 0,
        "Y" => 1,
        _ => 2,
    };

    let outcome_score = match ((you_index + 3) - opp_index) % 3 {
        0 => 3,
        1 => 6,
        _ => 0,
    };

    you_index + 1 + outcome_score
}

fn score_part_2(line: &String) -> usize {
    let goes = line.split(' ').take(2).collect::<Vec<&str>>();
    let [opp, you] = <[&str; 2]>::try_from(goes).ok().unwrap();

    let opp_index = match opp {
        "A" => 0,
        "B" => 1,
        _ => 2,
    };

    let required_outcome = match you {
        "X" => 0,
        "Y" => 1,
        _ => 2,
    };

    let your_index = (opp_index + 2 + required_outcome) % 3;

    your_index + 1 + (required_outcome * 3)
}
