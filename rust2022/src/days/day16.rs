use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::{common::get_lines, Args};

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
    players_remaining: u8,
    ttl: usize,
    open: usize,
    initial_ttl: usize,
    initial_pos: usize,
}

impl State {
    fn is_valve_open(&self, valve: &Valve) -> bool {
        let bit_mask = 2 << valve.index;

        self.open & bit_mask != 0
    }
}

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day16-test")
    } else {
        get_lines("day16")
    };

    let mut name_index_map: HashMap<&str, usize> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let name = &line[6..8];
        name_index_map.insert(name, index);
    }

    let valves: Vec<Valve> = lines
        .iter()
        .map(|x| Valve::from_str(x, &name_index_map))
        .collect();

    let matrix: HashMap<(usize, usize), usize> = build_distance_matrix(&valves);
    let mut cache = HashMap::new();

    // don't consider any valves with a 0 flow
    let flow_valves = valves.iter().filter(|x| x.rate > 0).collect::<Vec<_>>();

    let start_pos = name_index_map.get("AA").unwrap().to_owned();
    let best_solo = get_future_value(
        State {
            position: start_pos,
            ttl: 30,
            players_remaining: 0,
            open: 0,
            initial_ttl: 30,
            initial_pos: start_pos,
        },
        0,
        &mut 0,
        &flow_valves,
        &matrix,
        &mut cache,
    );

    let best_with_elephant = get_future_value(
        State {
            position: start_pos,
            ttl: 26,
            players_remaining: 1,
            open: 0,
            initial_ttl: 26,
            initial_pos: start_pos,
        },
        0,
        &mut 0,
        &flow_valves,
        &matrix,
        &mut cache,
    );

    if !args.no_answers {
        println!(
            "Day 16, Part 1: The most I can release in 30 minutes is {}",
            best_solo
        );
        println!(
            "Day 16, Part 2: The most I can release with the elephant in 26 minutes is {}",
            best_with_elephant
        );
    }

    (best_solo.to_string(), best_with_elephant.to_string())
}

fn get_future_value(
    state: State,
    curr_value: usize,
    lower_bound: &mut usize,
    valves: &Vec<&Valve>,
    distances: &HashMap<(usize, usize), usize>,
    value_cache: &mut HashMap<State, usize>,
) -> usize {
    // if we've already been in this state before, return how valuable it was last
    if let Some(cache_value) = value_cache.get(&state) {
        *lower_bound = max(*lower_bound, curr_value + *cache_value);

        return *cache_value;
    }

    // we need at least three ticks to move, open, and reap the reward from pressure release
    let time_expired = state.ttl < 3;
    if time_expired {
        // the volcano's erupted, we can't improve, I hope I've done enough!
        *lower_bound = max(*lower_bound, curr_value);

        return 0;
    }

    // what's a cheap upper bound for how we could improve this state?
    let upper_bound = upper_bound(&state, valves, distances);
    if upper_bound + curr_value < *lower_bound {
        // this option can't improve on the best we've found
        return 0;
    }

    // enumerate the possible actions and see what the most valuable action is
    let mut best_increase = 0;

    // go through all the valves and try opening them!
    for try_valve in valves {
        // have I been here?
        if state.is_valve_open(try_valve) {
            // we've already opened this one, no reason to open it again
            continue;
        }

        let distance = distances[&(state.position, try_valve.index)];
        let time_to_open = distance + 1;

        // do we have time to move here and open the valve and let that valve run for at least 1 tick?
        if state.ttl <= time_to_open {
            // no, don't bother trying this one
            continue;
        }

        let new_ttl = state.ttl - time_to_open;
        let try_state = State {
            position: try_valve.index,
            ttl: new_ttl,
            open: state.open + (2 << try_valve.index), // "close" the valve we're trying!
            ..state
        };

        let valve_value = new_ttl * try_valve.rate;

        let future_value = get_future_value(
            try_state,
            curr_value + valve_value,
            lower_bound,
            valves,
            distances,
            value_cache,
        );
        best_increase = max(best_increase, future_value + valve_value);
    }

    // add a "do nothing" option where we immediately pass the buck to the next player
    if state.players_remaining > 0 {
        let future_value = get_future_value(
            State {
                position: state.initial_pos,
                ttl: state.initial_ttl,
                players_remaining: state.players_remaining - 1,
                ..state
            },
            curr_value,
            lower_bound,
            valves,
            distances,
            value_cache,
        );

        best_increase = max(best_increase, future_value);
    }

    value_cache.insert(state, best_increase);

    best_increase
}

fn build_distance_matrix(valves: &Vec<Valve>) -> HashMap<(usize, usize), usize> {
    let mut matrix: HashMap<(usize, usize), usize> = HashMap::new();

    for a in valves {
        for b in valves {
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

fn shortest_distance_between(a: usize, b: usize, valves: &Vec<Valve>) -> usize {
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
            let valve = &valves[*indx];

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

fn upper_bound(
    state: &State,
    valves: &Vec<&Valve>,
    distances: &HashMap<(usize, usize), usize>,
) -> usize {
    let mut upper_bound = 0;

    for valve in valves {
        if state.is_valve_open(valve) {
            continue;
        }

        let (ttl, position) = if state.players_remaining > 0 {
            // we don't really know when the valves could be opened,
            // so assume they're opened ASAP from the start
            (state.initial_ttl, state.initial_pos)
        } else {
            (state.ttl, state.position)
        };

        let distance = distances[&(position, valve.index)];
        let time_to_open = distance + 1;

        if ttl <= time_to_open {
            continue;
        }

        let new_ttl = ttl - time_to_open;

        upper_bound += new_ttl * valve.rate;
    }

    upper_bound
}
