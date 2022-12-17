use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::common::get_lines;

#[derive(Debug)]
struct Valve {
    index: usize,
    rate: usize,
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
            .map(|m| m.as_str().parse::<usize>().unwrap())
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
    ttl: usize,
    open: usize,
}

pub fn run(_: bool) {
    let lines = get_lines("day16");

    let mut name_index_map: HashMap<&str, usize> = HashMap::new();
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
    let start = State {
        position: name_index_map.get("AA").unwrap().to_owned(),
        ttl: time_to_eruption,
        open: 0,
    };

    let matrix: HashMap<(usize, usize), usize> = build_distance_matrix(&valves);

    let mut cache = HashMap::new();
    let best = get_future_value(start, &valves, &matrix, &mut cache);

    println!("Cache size: {}", cache.len());

    println!(
        "Day 16, Part 1: The most I can release in {} minutes is {}",
        time_to_eruption, best
    );
}

fn get_future_value(
    state: State,
    valves: &HashMap<usize, Valve>,
    distances: &HashMap<(usize, usize), usize>,
    value_cache: &mut HashMap<State, usize>,
) -> usize {
    // if we've already been in this state before, return how valuable it was last
    if let Some(cache_value) = value_cache.get(&state) {
        return cache_value.to_owned();
    }

    let time_expired = state.ttl <= 0;

    if time_expired {
        // the volcano's erupted, we can't improve, I hope I've done enough!

        return 0;
    }

    // enumerate the possible actions and see what the most valuable action is
    let mut best_increase = 0;

    // go through all closed valves and try opening them!
    for (_, valve) in valves {
        // have I been here?
        let bit_mask = 2 << valve.index;
        if state.open & bit_mask > 0 {
            // we've already opened this one, no reason to open it again
            continue;
        }

        let try_valve = valves.get(&valve.index).unwrap();
        if try_valve.rate == 0 {
            // this has no flow! Don't waste time opening it...
            continue;
        }

        let distance = distances[&(state.position, try_valve.index)];

        // do we have time to move here and open the valve and let that valve run for at least a minute?
        if state.ttl < distance + 1 {
            // no, don't bother trying this one
            continue;
        }

        // how long we'll have left over after traveling + opening the valve
        let ttl = state.ttl - distance - 1;

        // let's try moving to this valve!
        let try_state = State {
            position: try_valve.index,
            ttl,
            open: state.open + bit_mask, // "close" the valve we're trying!
        };

        let future_value = get_future_value(try_state, valves, distances, value_cache);
        best_increase = max(best_increase, future_value + (ttl * try_valve.rate));
    }

    // // can we open the current valve? Is it worth it?
    // if at_valve.rate > 0 && (state.open & bit_mask) == 0 {
    //     // try opening the current state
    //     let try_state = State {
    //         open: state.open + bit_mask,
    //         ttl: state.ttl - 1,
    //         position: state.position,
    //     };

    //     let future_value = get_future_value(try_state, valves, distances, value_cache);
    //     let valve_open_value = at_valve.rate * (state.ttl - 1);
    //     best_increase = max(best_increase, valve_open_value + future_value);
    // }

    // // can we go to a different valve?
    // for valve_index in &at_valve.leads_to {
    //     let try_state = State {
    //         position: valve_index.to_owned(),
    //         ttl: state.ttl - 1,
    //         open: state.open,
    //     };

    //     let future_value = get_future_value(try_state, valves, distances, value_cache);
    //     best_increase = max(best_increase, future_value);
    // }

    value_cache.insert(state, best_increase);

    best_increase
}

fn build_distance_matrix(valves: &HashMap<usize, Valve>) -> HashMap<(usize, usize), usize> {
    let mut matrix: HashMap<(usize, usize), usize> = HashMap::new();

    for (_, a) in valves {
        for (_, b) in valves {
            // we only need to do one direction, then trivially duplicate it
            if a.index <= b.index {
                continue;
            }

            let dist = shortest_distance_between(a.index, b.index, &valves);

            matrix.insert((a.index, b.index), dist);
            matrix.insert((b.index, a.index), dist);
        }
    }

    matrix
}

fn shortest_distance_between(a: usize, b: usize, valves: &HashMap<usize, Valve>) -> usize {
    let mut dist = 0;
    let mut wave: Vec<&usize> = vec![&a];
    let mut visited = HashSet::new();
    visited.insert(a);

    loop {
        dist += 1;

        // the distance must always be less than the number of valves
        if dist > (valves.len() * 2) {
            break;
        }

        let mut next_wave = Vec::new();

        for indx in wave {
            let valve = valves.get(&indx).unwrap();

            for neighbour in &valve.leads_to {
                // if we've already visited this valve,
                // no reason to see it again
                if visited.contains(&neighbour) {
                    continue;
                }

                if neighbour == &b {
                    return dist;
                } else {
                    visited.insert(neighbour.to_owned());
                    next_wave.push(neighbour);
                }
            }
        }

        // we've exhausted our neighbours! How is that!?
        if next_wave.len() == 0 {
            break;
        }

        wave = next_wave;
    }

    println!("We should always have a route between any two valves...");
    panic!()
}
