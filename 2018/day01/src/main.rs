use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part 1:\n{}", part1());
    println!("Part 2:\n{}", part2());
}

fn part1() -> i32 {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|op| op.parse::<i32>().unwrap())
        .fold(0, std::ops::Add::add)
}

fn part2() -> i32 {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let operations: Vec<i32> = contents
        .lines()
        .map(|op| op.parse::<i32>().unwrap())
        .collect();
    let mut seen = HashSet::new();
    let mut frequency = 0;

    loop {
        for operation in operations.iter() {
            frequency += operation;

            if seen.contains(&frequency) {
                return frequency;
            }

            seen.insert(frequency);
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_sum() {
        assert_eq!(3, 3);
    }
}
