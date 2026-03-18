use crate::common::{get_lines, Answer};
use crate::maps::Vector;
use crate::Args;
use num::integer::Roots;
use std::collections::{HashMap, HashSet, VecDeque};

type Map = MapContainer;
type AdjacencyMap = HashMap<Vector, Vector>;

#[derive(Clone, Debug)]
enum FieldElement {
    Mirror(AdjacencyMap),     // map of (input : output)
    Splitter(Vector, Vector), // where the splitter will point a split element
}

#[derive(Debug)]
struct MapContainer {
    state: HashMap<Vector, FieldElement>,
    raw: Vec<Vec<char>>,
    size: (usize, usize),
}

impl MapContainer {
    fn at_pos(&self, pos: &Vector) -> Option<&FieldElement> {
        self.state.get(pos)
    }
}

impl FromIterator<Vec<char>> for MapContainer {
    fn from_iter<T: IntoIterator<Item = Vec<char>>>(iter: T) -> Self {
        let chars: Vec<Vec<char>> = iter.into_iter().collect();
        let size = (chars.len(), chars[0].len());

        MapContainer {
            state: build_adjacency_map(chars.clone()),
            raw: chars,
            size,
        }
    }
}

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day16-test")
    } else {
        get_lines("day16")
    };

    let map = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<MapContainer>();

    let top_left = (Vector { row: 0, col: -1 }, Vector { row: 0, col: 1 });

    let (height, width) = (lines.len() as isize, lines[0].len() as isize);
    let max = Vector {
        row: height - 1,
        col: width - 1,
    };

    let visited_from_top_left = energise_map2(&map, top_left, &max);

    // TODO there's still plenty to be improved here, we're repeating a lot of graph traversal unnecessarily
    let best_row = (0..height)
        .into_iter()
        .map(|i| {
            let left_entry = (Vector { row: i, col: 0 }, Vector { row: 0, col: 1 });
            let right_entry = (
                Vector {
                    row: i,
                    col: width - 1,
                },
                Vector { row: 0, col: -1 },
            );

            energise_map2(&map, left_entry, &max).max(energise_map2(&map, right_entry, &max))
        })
        .reduce(|a, b| a.max(b))
        .unwrap();

    let best_col = (0..width)
        .into_iter()
        .map(|i| {
            let top_entry = (Vector { row: 0, col: i }, Vector { row: 1, col: 0 });
            let bottom_entry = (
                Vector {
                    row: height - 1,
                    col: i,
                },
                Vector { row: -1, col: 0 },
            );

            let best_top = energise_map2(&map, top_entry, &max);
            let best_bottom = energise_map2(&map, bottom_entry, &max);

            best_top.max(best_bottom)
        })
        .reduce(|a, b| a.max(b))
        .unwrap();

    (
        visited_from_top_left.to_string(),
        best_row.max(best_col).to_string(),
    )
}

fn energise_map2(map: &Map, start_from: (Vector, Vector), max: &Vector) -> u64 {
    let start_target = find_next_non_empty(&map.raw, &start_from.0, start_from.1);

    let mut rays = VecDeque::with_capacity(map.state.len().sqrt());
    rays.push_back((start_target, start_from.0));

    let mut visited = vec![0; map.size.0 * map.size.1];

    extend(&mut visited, &start_from.0, &start_target, max);

    let mut actioned = HashSet::with_capacity(map.state.len());

    while let Some((current, coming_from)) = rays.pop_front() {
        let Some(element) = map.at_pos(&current) else {
            continue;
        };

        match element {
            FieldElement::Mirror(adjacency_map) => {
                if let Some(goes_to) = adjacency_map.get(&coming_from.dir(&current)) {
                    extend(&mut visited, &current, goes_to, max);

                    let ray = (*goes_to, current);
                    if !actioned.contains(&ray) {
                        rays.push_back(ray);
                        actioned.insert(ray);
                    }
                }
            }
            FieldElement::Splitter(one, two) => {
                extend(&mut visited, &current, one, max);
                extend(&mut visited, &current, two, max);

                let ray_one = (*one, current);
                let ray_two = (*two, current);

                if !actioned.contains(&ray_one) {
                    rays.push_back(ray_one);
                    rays.push_back(ray_two);

                    actioned.insert(ray_one);
                    actioned.insert(ray_two);
                }
            }
        }
    }

    visited.iter().sum::<u64>()
}

fn extend(visited: &mut Vec<u64>, from: &Vector, to: &Vector, max: &Vector) {
    let (min_row, max_row) = (
        from.row.min(to.row).max(0),
        from.row.max(to.row).min(max.row),
    );
    let (min_col, max_col) = (
        from.col.min(to.col).max(0),
        from.col.max(to.col).min(max.col),
    );

    (min_row..=max_row)
        .flat_map(|row| (min_col..=max_col).map(move |col| Vector { row, col }))
        .for_each(|v| visited[((v.row * (max.col + 1)) + v.col) as usize] = 1);
}

fn build_adjacency_map(chars: Vec<Vec<char>>) -> HashMap<Vector, FieldElement> {
    // for each non-empty element, find the two elements that are adjacent to it
    let mut output = HashMap::new();

    let from_up = Vector { row: -1, col: 0 };
    let from_down = Vector { row: 1, col: 0 };
    let from_left = Vector { row: 0, col: -1 };
    let from_right = Vector { row: 0, col: 1 };

    for (row, line) in chars.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            let pos = Vector {
                row: row as isize,
                col: col as isize,
            };

            match char {
                &'.' => {} // already empty, do nothing
                &'|' => {
                    let up = find_next_non_empty(&chars, &pos, from_up);
                    let down = find_next_non_empty(&chars, &pos, from_down);

                    output.insert(pos, FieldElement::Splitter(up, down));
                }
                &'-' => {
                    let left = find_next_non_empty(&chars, &pos, from_left);
                    let right = find_next_non_empty(&chars, &pos, from_right);

                    output.insert(pos, FieldElement::Splitter(left, right));
                }
                &'/' => {
                    let up = find_next_non_empty(&chars, &pos, from_up);
                    let down = find_next_non_empty(&chars, &pos, from_down);
                    let left = find_next_non_empty(&chars, &pos, from_left);
                    let right = find_next_non_empty(&chars, &pos, from_right);

                    let map = HashMap::from([
                        (from_up, left),
                        (from_left, up),
                        (from_down, right),
                        (from_right, down),
                    ]);

                    output.insert(pos, FieldElement::Mirror(map));
                }
                &'\\' => {
                    let up = find_next_non_empty(&chars, &pos, from_up);
                    let down = find_next_non_empty(&chars, &pos, from_down);
                    let left = find_next_non_empty(&chars, &pos, from_left);
                    let right = find_next_non_empty(&chars, &pos, from_right);

                    let map = HashMap::from([
                        (from_up, right),
                        (from_right, up),
                        (from_down, left),
                        (from_left, down),
                    ]);

                    output.insert(pos, FieldElement::Mirror(map));
                }
                &_ => panic!("unexpected char"),
            }
        }
    }

    output
}

fn find_next_non_empty(state: &Vec<Vec<char>>, from: &Vector, vector: Vector) -> Vector {
    let mut curr = from.add(&vector);

    while curr.row >= 0
        && curr.col >= 0
        && curr.row < state.len() as isize
        && curr.col < state[curr.row as usize].len() as isize
    {
        // is this a piece?
        let piece = state[curr.row as usize][curr.col as usize];

        match piece {
            '.' => {} // empty space, keep moving
            '|' => {
                if vector.row == 0 {
                    return curr;
                } // else we pass right through
            }
            '-' => {
                if vector.col == 0 {
                    return curr;
                } // else we pass right through
            }
            '/' | '\\' => return curr,
            _ => panic!("unexpected char"),
        };

        curr = curr.add(&vector);
    }

    curr
}
