use std::{
    cmp::{max, min},
    collections::HashMap,
};

use regex::Regex;

use crate::{common::get_lines, Args};

#[derive(Debug)]
struct Blueprint {
    index: i32,
    costs: [[i32; 4]; 4],
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    ttl: i32,
    resources: [i32; 4],
    robots: [i32; 4],
}

static mut GLOBAL_BEST: i32 = 0;

pub fn run(args: &Args) {
    let lines = if args.test {
        get_lines("day19-test")
    } else {
        get_lines("day19")
    };

    let blueprints = lines.iter().map(build_blueprint).collect::<HashMap<_, _>>();

    let mut part_1_total = 0;
    let mut part_2_total = 1;

    for index in 1..=blueprints.len() {
        let blueprint = blueprints.get(&(index as i32)).unwrap();
        let value = best_value_for_ttl(24, &blueprint);

        part_1_total += value * blueprint.index;
    }

    for index in 1..=min(3, blueprints.len()) {
        let blueprint = blueprints.get(&(index as i32)).unwrap();
        let value = best_value_for_ttl(32, &blueprint);

        part_2_total *= value;
    }

    if !args.no_answers {
        println!(
            "Day 19, Part 1: Quality level sum is {} after {} minutes",
            part_1_total, 24
        );
        println!(
            "Day 19, Part 2: Quality level product is {} for the first three blueprints after {} minutes",
            part_2_total, 32
        );
    }
}

fn best_value_for_ttl(ttl: i32, blueprint: &Blueprint) -> i32 {
    unsafe {
        GLOBAL_BEST = 0;
    }

    let start = State {
        ttl,
        resources: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };
    let mut cache = HashMap::new();

    best_value(start, blueprint, &mut cache)
}

fn best_value(state: State, blueprint: &Blueprint, cache: &mut HashMap<State, i32>) -> i32 {
    if let Some(value) = cache.get(&state) {
        return value.to_owned();
    }

    let do_nothing_value = state.resources[3] + (state.robots[3] * state.ttl);

    // if we've only got one tick remaining, there's no point building more robots
    // so just return the number of geodes we have + number of geode robots at this point
    if state.ttl <= 1 {
        return do_nothing_value;
    }

    // if we have enough robots to perpetually create geode robots, just do that
    if state.robots[2] >= blueprint.costs[3][2]
        && state.resources[2] >= blueprint.costs[3][2]
        && state.resources[0] >= blueprint.costs[3][0]
        && state.robots[0] >= blueprint.costs[3][0]
    {
        // geodes we currently have
        // + geode robots * time left
        // + perpetually creating geode robots and how much they'll harvest
        return do_nothing_value + ((state.ttl * (state.ttl - 1)) / 2);
    }

    // work out the upper bound for our state, if we have a higher value already then we can trivially return
    let upper_bound = upper_bound(&state);
    unsafe {
        if upper_bound <= GLOBAL_BEST {
            return 0;
        }
    }

    let mut best = 0;

    // permute possible actions and determine best

    for r in 0..state.robots.len() {
        // can I create this robot?

        if let Some(new_state) = wait_for_robot(&state, blueprint, r) {
            // I can, if I wait until this state... try it out!
            best = max(best, best_value(new_state, blueprint, cache));
        }
    }

    best = max(best, do_nothing_value);

    // cache the best value we could make at this state
    cache.insert(state, best);

    unsafe {
        GLOBAL_BEST = max(GLOBAL_BEST, best);
    }

    best
}

// create a sensible upper bound for a current state
fn upper_bound(state: &State) -> i32 {
    // one reasonable upper bound is to assume the state can perpetually create new geode robots from this position
    // the smaller we can make this upper bound, the more efficient our solution will be!
    return state.resources[3]
        + (state.robots[3] * state.ttl)
        + ((state.ttl * (state.ttl - 1)) / 2);
}

fn build_blueprint(line: &String) -> (i32, Blueprint) {
    let re = Regex::new(r"\d+").unwrap();
    let captures = re.captures_iter(&line).collect::<Vec<_>>();

    let numbers = captures
        .iter()
        .take(7)
        .map(|n| n.get(0).unwrap().as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [index, ore, clay, obs_1, obs_2, geode_1, geode_2] =
        <[i32; 7]>::try_from(numbers).ok().unwrap();

    (
        index,
        Blueprint {
            index,
            costs: [
                [ore, 0, 0, 0],
                [clay, 0, 0, 0],
                [obs_1, obs_2, 0, 0],
                [geode_1, 0, geode_2, 0],
            ],
        },
    )
}

fn wait_for_robot(state: &State, blueprint: &Blueprint, r: usize) -> Option<State> {
    let mut wait_time = 0;
    for res in 0..=3 {
        // do we need this resource?
        if blueprint.costs[r][res] <= 0 {
            continue;
        }

        // if we don't have a robot for this resource type, we can't wait for it!
        if state.robots[res] == 0 {
            return None;
        }

        // okay, we have a robot, how long would it take to get enough resource?
        let resources_needed = blueprint.costs[r][res] - state.resources[res];
        wait_time = max(wait_time, div_ceil(resources_needed, state.robots[res]));
    }

    // we need to wait for resources... and build this robot!
    wait_time += 1;

    // we don't have enough time to wait for this robot _and_ benefit from it
    if state.ttl < wait_time + 1 {
        return None;
    }

    // we have a wait time... let's return what that state would look like
    let resources: [i32; 4] = state
        .resources
        .iter()
        .enumerate()
        .map(|(i, res)| res + (state.robots[i] * wait_time) - blueprint.costs[r][i])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut robots = state.robots;
    robots[r] += 1;

    Some(State {
        ttl: state.ttl - wait_time,
        resources,
        robots,
    })
}

fn div_ceil(numerator: i32, denominator: i32) -> i32 {
    // abuses the natural floor of regular division to create a ceil division
    (numerator + denominator - 1) / denominator
}
