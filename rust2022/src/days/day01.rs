use crate::{common::get_file_contents, Args};

pub fn run(args: &Args) -> (String, String) {
    let input = get_file_contents("day01");

    let groups = input.split("\n\n");
    let mut max = 0;
    let mut all_totals: Vec<i64> = Vec::new();

    for group in groups {
        let total_calories = group
            .split("\n")
            .flat_map(|x| x.parse::<i64>())
            .reduce(|x, y| x + y)
            .expect("Uh oh! Failed to work out total calories for this elf");

        all_totals.push(total_calories);

        max = if total_calories > max {
            total_calories
        } else {
            max
        };
    }

    all_totals.sort_by(|x, y| y.cmp(x));

    let sum_highest_three = &all_totals[0..3]
        .iter()
        .map(|x| x.to_owned())
        .reduce(|x, y| x + y)
        .expect("Uh oh! Can't find calory sum of top three elves");

    if !args.no_answers {
        println!("Day 1, Part 1: The elf with the largest number of calories has {max}");
        println!("Day 1, Part 2: The sum of the three most calorific elves is {sum_highest_three}");
    }

    ("".to_string(), "".to_string())
}
