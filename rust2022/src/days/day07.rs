use std::{cmp::min, collections::HashMap};

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    // ignore the first line, this just centres us at `/`
    let lines = &get_lines("day07")[1..];

    let mut sizes: HashMap<Vec<&str>, (usize, Vec<Vec<&str>>)> = HashMap::new();
    let mut path: Vec<&str> = Vec::new();
    let mut current_path_size: usize = 0;
    let mut current_path_contents: Vec<Vec<&str>> = Vec::new();

    let mut last_command_was_ls = false;

    for line in lines {
        if is_command(line) {
            if last_command_was_ls {
                sizes.insert(
                    path.clone(),
                    (current_path_size, current_path_contents.clone()),
                );
            }

            last_command_was_ls = false;

            if line == "$ ls" {
                last_command_was_ls = true;
                current_path_size = 0;
                current_path_contents = Vec::new();
            } else {
                // it's a CD, where are we moving too?

                let (_, to) = line
                    .split_once("$ cd ")
                    .expect("Failed to split the cd command");

                // move our pointer
                if to == "/" {
                    path = Vec::new();
                } else if to == ".." {
                    path.pop();
                } else {
                    path.push(to);
                }
            }
        } else {
            let (size, name) = line
                .split_once(' ')
                .expect("Failed to split non-command line");

            if size == "dir" {
                let mut contained_dir = path.clone();
                contained_dir.push(name);
                current_path_contents.push(contained_dir);
            } else {
                let file_size = size.parse::<usize>().unwrap();
                current_path_size += file_size;
            }
        }
    }

    sizes.insert(
        path.clone(),
        (current_path_size, current_path_contents.clone()),
    );

    let part_1_limit = 100000;
    let mut part_1_sum = 0;

    for path in sizes.keys() {
        let path_size = size_at_path(path, &sizes);

        if path_size <= part_1_limit {
            part_1_sum += path_size;
        }
    }

    let total_disk_space = 70000000;
    let updgrade_disk_needed = 30000000;
    let mut disk_space_used = 0;

    for path in sizes.keys() {
        if path.len() == 1 {
            let path_size = size_at_path(path, &sizes);
            disk_space_used += path_size;
        }
    }

    let mut smallest_path = total_disk_space;

    let disk_space_needed = updgrade_disk_needed - (total_disk_space - disk_space_used);

    for path in sizes.keys() {
        let path_size = size_at_path(path, &sizes);

        if path_size > disk_space_needed {
            smallest_path = min(smallest_path, path_size);
        }
    }

    if !args.no_answers {
        println!(
            "Day 7, Part 1: The size of directories less than 100k is {}",
            part_1_sum
        );
        println!(
            "Day 7, Part 2: The smallest path I can delete to free up enough room is {}",
            smallest_path
        );
    }

    ("".to_string(), "".to_string())
}

fn is_command(line: &String) -> bool {
    line.as_bytes()[0] as char == '$'
}

fn size_at_path(
    dir: &Vec<&str>,
    file_system: &HashMap<Vec<&str>, (usize, Vec<Vec<&str>>)>,
) -> usize {
    let (size, subdirs) = file_system.get(dir).expect("Can't find value of path");

    let mut total_size = size.to_owned();

    for subdir in subdirs.iter() {
        total_size += size_at_path(subdir, file_system);
    }

    total_size
}
