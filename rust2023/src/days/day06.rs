use crate::Args;

pub fn run(args: &Args) -> (String, String) {
    let race_records: Vec<(isize, isize)> = if args.test {
        vec![(7, 9), (15, 40), (30, 200)]
    } else {
        vec![(49, 298), (78, 1185), (79, 1066), (80, 1181)]
    };

    let part_1: isize = race_records
        .iter()
        .map(|(time, distance)| num_ways_to_win(*time, *distance))
        .product();

    let (part_2_time, part_2_distance) = smash_numbers(race_records);
    let part_2: isize = num_ways_to_win(part_2_time, part_2_distance);

    if !args.no_answers {
        println!("Day 6, Part 1: The answer is {part_1}");
        println!("Day 6, Part 2: The answer is {part_2}");
    }

    (part_1.to_string(), "".to_string())
}

fn smash_numbers(numbers: Vec<(isize, isize)>) -> (isize, isize) {
    let time = numbers
        .iter()
        .map(|(time, _)| time.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<isize>()
        .unwrap();

    let distance = numbers
        .iter()
        .map(|(_, distance)| distance.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<isize>()
        .unwrap();

    (time, distance)
}

fn num_ways_to_win(time: isize, distance: isize) -> isize {
    let (lower, upper) = solve_quadratic(1f64, (time * -1) as f64, distance as f64);

    let min_acceleration_time = lower.trunc() as isize + 1;
    let max_acceleration_time = upper.ceil() as isize - 1;

    max_acceleration_time - min_acceleration_time + 1
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = ((b * b) - (4f64 * a * c)).sqrt();

    (
        ((b * -1f64) - root) / (2f64 * a),
        ((b * -1f64) + root) / (2f64 * a),
    )
}
