use crate::common::get_file_contents;

pub fn run() {
    let file = get_file_contents("day05");

    let parts = file.split("\n\n").take(2).collect::<Vec<&str>>();
    let [crates_pattern, commands] = <[&str; 2]>::try_from(parts).ok().unwrap();

    println!("{}", crates_pattern);

    let num_columns = 9;
    let mut crates: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_columns {
        crates.insert(0, Vec::new());
    }

    for line in crates_pattern.lines() {
        if line == " 1   2   3   4   5   6   7   8   9 " {
            continue;
        }

        for i in 0..num_columns {
            let crate_index = (i * 4) + 1;
            let box_type = line.as_bytes()[crate_index] as char;

            if box_type == ' ' {
                continue;
            }

            crates[i].push(box_type);
        }
    }

    for command in commands.lines() {
        let command_parts = command.split(' ').take(6).collect::<Vec<&str>>();
        let [_, num, _, from, _, to] = <[&str; 6]>::try_from(command_parts).ok().unwrap();
        let from_int: usize = from.parse().unwrap();
        let to_int: usize = to.parse().unwrap();

        for _ in 0..num.parse::<usize>().unwrap() {
            let removed = crates[from_int - 1].remove(0);

            crates[to_int - 1].insert(0, removed)
        }
    }

    let mut answer = String::from("");
    for i in 0..num_columns {
        answer.push(crates[i][0]);
    }

    println!("Part 1: the first box on each column spells: {}", answer);
}
