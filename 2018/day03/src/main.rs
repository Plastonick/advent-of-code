use regex::{CaptureMatches, Regex};
use std::{collections::HashMap, fs};

struct Claim {
    id: i32,
    left_offset: i32,
    top_offset: i32,
    width: i32,
    height: i32,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    let claims = parse_contents(contents);

    let overlaps = part1(&claims);
    let id = part2(&claims);

    println!("Part 1: {}", overlaps);
    println!("Part 2: {}", id.unwrap_or_default());
}

fn parse_contents(contents: String) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    contents
        .lines()
        .map(|line| extract_captures(re.captures_iter(line)))
        .collect()
}

fn part2(claims: &Vec<Claim>) -> Option<i32> {
    for claim in claims {
        if !has_intersection(claim, claims) {
            return Some(claim.id);
        }
    }

    None
}

fn has_intersection(a: &Claim, claims: &Vec<Claim>) -> bool {
    for comp in claims {
        if a.id == comp.id {
            continue;
        }

        if intersects(&a, &comp) {
            return true;
        }
    }

    false
}

fn intersects(a: &Claim, b: &Claim) -> bool {
    if a.left_offset + a.width < b.left_offset {
        return false;
    }

    if b.left_offset + b.width < a.left_offset {
        return false;
    }

    if a.top_offset + a.height < b.top_offset {
        return false;
    }

    if b.top_offset + b.height < a.top_offset {
        return false;
    }

    true
}

fn part1(claims: &Vec<Claim>) -> i32 {
    let mut fabric: HashMap<(i32, i32), i32> = HashMap::new();
    for claim in claims {
        for i in claim.left_offset..claim.left_offset + claim.width {
            for j in claim.top_offset..claim.top_offset + claim.height {
                if !fabric.contains_key(&(i, j)) {
                    fabric.insert((i, j), 1);
                } else {
                    fabric.insert((i, j), fabric.get(&(i, j)).unwrap() + 1);
                }
            }
        }
    }

    fabric
        .values()
        .filter(|&&x| x > { 1 as i32 })
        .count()
        .try_into()
        .unwrap()
}

fn extract_captures(captures: CaptureMatches) -> Claim {
    for cap in captures {
        return Claim {
            id: cap[1].parse::<i32>().unwrap(),
            left_offset: cap[2].parse::<i32>().unwrap(),
            top_offset: cap[3].parse::<i32>().unwrap(),
            width: cap[4].parse::<i32>().unwrap(),
            height: cap[5].parse::<i32>().unwrap(),
        };
    }

    Claim {
        id: 0,
        left_offset: 0,
        top_offset: 0,
        width: 0,
        height: 0,
    }
}
