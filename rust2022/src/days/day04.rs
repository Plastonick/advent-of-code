use crate::common::get_lines;

pub fn run(_: bool) {
    let lines = get_lines("day04");
    let mut total = 0;
    let mut captured = 0;
    let mut overlaps = 0;

    for line in lines {
        let groups = line.split(",").take(2).collect::<Vec<&str>>();
        let [first, second] = <[&str; 2]>::try_from(groups).ok().unwrap();

        let (first_start, first_end) = get_range(first);
        let (second_start, second_end) = get_range(second);

        let overlap = second_start <= first_end && second_end >= first_start;
        let first_contained_by_second = first_start >= second_start && first_end <= second_end;
        let second_contained_by_first = second_start >= first_start && second_end <= first_end;

        if first_contained_by_second || second_contained_by_first {
            captured += 1;
        }

        if overlap {
            overlaps += 1;
        }

        total += 1;
    }

    println!("Day 4, Part 1: Captured {captured} out of a total of {total}");
    println!("Day 4, Part 2: Overlaps {overlaps} out of a total of {total}");
}

fn get_range(group: &str) -> (i32, i32) {
    let range = group
        .split("-")
        .take(2)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let [start, end] = <[i32; 2]>::try_from(range).ok().unwrap();

    return (start, end);
}
