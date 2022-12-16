use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::common::get_lines;

#[derive(Debug)]
struct Valve {
    index: usize,
    rate: isize,
    leads_to: HashSet<usize>,
}

impl Valve {
    pub fn from_str(string: &str, map: &HashMap<&str, usize>) -> Valve {
        let pattern: Regex =
            Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                .unwrap();

        let captures = pattern.captures(string).unwrap();
        let index = map
            .get(captures.get(1).map(|m| m.as_str()).unwrap())
            .unwrap()
            .to_owned();
        let rate = captures
            .get(2)
            .map(|m| m.as_str().parse::<isize>().unwrap())
            .unwrap();

        let mut leads_to = HashSet::new();
        let lead_str = captures.get(3).map(|m| m.as_str()).unwrap();
        for lead in lead_str.split(", ") {
            leads_to.insert(map.get(lead).unwrap().to_owned());
        }

        Valve {
            index,
            rate,
            leads_to,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    position: usize,
    ttl: isize,
    open_valves: Vec<usize>,
}

pub fn run(_: bool) {
    let lines = get_lines("day16");

    // let mut valves = Vec::new();
    let mut name_index_map = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let name = &line[6..8];
        name_index_map.insert(name, index);
    }

    let valves: HashMap<usize, Valve> = lines
        .iter()
        .map(|x| {
            let valve = Valve::from_str(x, &name_index_map);
            (valve.index.to_owned(), valve)
        })
        .collect();

    let time_to_eruption = 30;

    // let mut cache = HashMap::new();
    // let best_value = get_best_value(
    //     (
    //         State {
    //             position: String::from("AA"),
    //             ttl: time_to_eruption,
    //             open: Vec::new(),
    //         },
    //         0,
    //     ),
    //     &valves,
    //     &mut cache,
    // );

    let start = State {
        position: name_index_map.get("AA").unwrap().to_owned(),
        ttl: time_to_eruption,
        open_valves: Vec::new(),
    };
    // let mut states = Vec::new();
    // states.push(start);

    // loop {
    //     for (state, released) in states.pop() {
    //         let time_expired = state.ttl <= 0;
    //         let all_valves_open = state.open.len() == valves.len();
    //         let best_possible = 50; // TODO

    //         if time_expired || all_valves_open || best_possible < best {
    //             best = released;

    //             cache.insert(state, released);

    //             continue;
    //         }

    //         let at_valve = valves.get(&state.position).unwrap();

    //         // can we open the current valve?
    //         if !state.open.contains(&state.position) {
    //             let mut now_open = state.open.clone(); // TODO clone is bad mkay
    //             now_open.push(at_valve.name.clone()); // TODO clone is bad mkay
    //             now_open.sort();

    //             // try opening the current state
    //             let try_state = State {
    //                 open: now_open,
    //                 ttl: state.ttl - 1,
    //                 position: state.position.clone(),
    //             };

    //             states.push((try_state, released + (at_valve.rate * state.ttl)));
    //         }

    //         // can we go to a different valve?
    //         for valve_name in &at_valve.leads_to {
    //             let try_state = State {
    //                 position: valve_name.clone(),
    //                 ttl: state.ttl - 1,
    //                 open: state.open.clone(), // TODO clone is bad mkay
    //             };

    //             states.push((try_state, released));
    //         }
    //     }

    //     // when we've not got any more states to look over
    //     if states.len() == 0 {
    //         break;
    //     }
    // }

    let mut cache = HashMap::new();
    let best = get_future_value(start, &valves, &mut cache);

    println!("Cache size: {}", cache.len());

    // TODO

    // Map out the entire tunnel system and remove any empty valves

    // TODO

    println!(
        "Day 16, Part 1: The most I can release in {} minutes is {}",
        time_to_eruption, best
    );
}

fn get_future_value(
    state: State,
    valves: &HashMap<usize, Valve>,
    value_cache: &mut HashMap<State, isize>,
) -> isize {
    // if we've already been in this state before, return how valuable it was last
    if let Some(cache_value) = value_cache.get(&state) {
        return cache_value.to_owned();
    }

    let time_expired = state.ttl <= 0;
    let all_valves_open = state.open_valves.len() == valves.len();

    if time_expired || all_valves_open {
        // the volcano's erupted, we can't improve, I hope I've done enough!

        return 0;
    }

    // enumerate the possible actions and see what the most valuable action is
    let at_valve = valves.get(&state.position).unwrap();
    let mut best_increase = 0;

    // can we open the current valve? Is it worth it?
    if at_valve.rate > 0 && !state.open_valves.contains(&state.position) {
        let mut now_open = state.open_valves.clone(); // TODO clone is bad mkay
        now_open.push(at_valve.index);

        // sort the list, this makes the caching worth it!
        now_open.sort();

        // try opening the current state
        let try_state = State {
            open_valves: now_open,
            ttl: state.ttl - 1,
            position: state.position,
        };

        let future_value = get_future_value(try_state, valves, value_cache);
        let valve_open_value = at_valve.rate * (state.ttl - 1);
        best_increase = max(best_increase, valve_open_value + future_value);
    }

    // can we go to a different valve?
    for valve_name in &at_valve.leads_to {
        let try_state = State {
            position: valve_name.to_owned(),
            ttl: state.ttl - 1,
            open_valves: state.open_valves.clone(), // TODO clone is bad mkay
        };

        let future_value = get_future_value(try_state, valves, value_cache);
        best_increase = max(best_increase, future_value);
    }

    value_cache.insert(state, best_increase);

    best_increase
}

fn build_distance_matrix(valves: &HashMap<usize, Valve>) -> HashMap<(usize, usize), usize> {
    let mut matrix: HashMap<(usize, usize), usize> = HashMap::new();

    for (_, a) in valves {
        for (_, b) in valves {
            if a.index == b.index {
                continue;
            }

            let dist = shortest_distance_between(a.index, b.index, &valves);

            matrix.insert((a.index, b.index), dist);
            matrix.insert((b.index, a.index), dist);
        }
    }

    dbg!(&matrix);

    matrix
}

fn shortest_distance_between(a: usize, b: usize, valves: &HashMap<usize, Valve>) -> usize {
    let mut dist = 0;
    let mut wave: Vec<&usize> = vec![&a];
    let mut visited = HashSet::new();
    visited.insert(a);

    loop {
        dist += 1;

        if dist > valves.len() {
            break;
        }

        let mut next_wave = Vec::new();

        for indx in wave.pop() {
            let valve = valves.get(&indx).unwrap();

            for neighbour in &valve.leads_to {
                if visited.contains(&neighbour) {
                    continue;
                }

                if neighbour.to_owned() == b {
                    return dist;
                } else {
                    visited.insert(neighbour.to_owned());
                    next_wave.push(neighbour);
                }
            }
        }

        wave = next_wave;
    }

    usize::MAX
}
