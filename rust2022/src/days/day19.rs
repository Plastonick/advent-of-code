use std::{cmp::max, collections::HashMap};

use regex::Regex;

use crate::common::get_lines;

#[derive(Debug)]
struct Blueprint {
    index: i32,
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    ttl: i32,
    ore: i32,
    ore_robots: i32,
    clay: i32,
    clay_robots: i32,
    obsidian: i32,
    obsidian_robots: i32,
    geodes: i32,
    geode_robots: i32,
}

static mut GLOBAL_BEST: i32 = 0;

pub fn run(test: bool) {
    let lines = if test {
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

    for index in 1..=3 {
        let blueprint = blueprints.get(&index).unwrap();
        let value = best_value_for_ttl(32, &blueprint);

        part_2_total *= value;
    }

    println!(
        "Day 19, Part 1: Quality level sum is {} after {} minutes",
        part_1_total, 24
    );
    println!(
        "Day 19, Part 2: Quality level product is {} for the first three blueprints after {} minutes",
        part_2_total, 32
    );
}

fn best_value_for_ttl(ttl: i32, blueprint: &Blueprint) -> i32 {
    unsafe {
        GLOBAL_BEST = 0;
    }

    let start = State {
        ttl,
        ore: 0,
        ore_robots: 1,
        clay: 0,
        clay_robots: 0,
        obsidian: 0,
        obsidian_robots: 0,
        geodes: 0,
        geode_robots: 0,
    };
    let mut cache = HashMap::new();

    let value = best_value(start, blueprint, &mut cache);

    value
}

fn best_value(state: State, blueprint: &Blueprint, cache: &mut HashMap<State, i32>) -> i32 {
    if let Some(value) = cache.get(&state) {
        return value.to_owned();
    }
    // if we've only got one tick remaining, there's no point building more robots
    // so just return the number of geodes we have + number of geode robots at this point
    if state.ttl == 1 {
        return state.geodes + state.geode_robots;
    }

    // if we have enough robots to perpetually create geode robots, just do that
    if state.obsidian_robots >= blueprint.geode.1
        && state.obsidian >= blueprint.geode.1
        && state.ore >= blueprint.geode.0
        && state.ore_robots >= blueprint.geode.0
    {
        // geodes we currently have
        // + geode robots * time left
        // + perpetually creating geode robots and how much they'll harvest
        return state.geodes
            + (state.geode_robots * state.ttl)
            + ((state.ttl * (state.ttl - 1)) / 2);
    }

    // work out the upper bound for our state, if we have a higher value already then we can trivially return
    let upper_bound = upper_bound(&state);
    unsafe {
        if upper_bound <= GLOBAL_BEST {
            return 0;
        }
    }

    // each robot will collect 1 piece of its resource
    let ore = state.ore + state.ore_robots;
    let clay = state.clay + state.clay_robots;
    let obsidian = state.obsidian + state.obsidian_robots;
    let geodes = state.geodes + state.geode_robots;
    let ttl = state.ttl - 1;

    let mut best = 0;

    // permute possible actions and determine best

    // 1. test creating an ore robot at this stage
    if state.ore >= blueprint.ore {
        let new_state = State {
            ore: ore - blueprint.ore,
            clay,
            obsidian,
            geodes,
            ttl,
            ore_robots: state.ore_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, blueprint, cache));
    }

    // 2. test creating a clay robot at this stage
    if state.ore >= blueprint.clay {
        let new_state = State {
            ore: ore - blueprint.clay,
            clay,
            obsidian,
            geodes,
            ttl,
            clay_robots: state.clay_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, blueprint, cache));
    }

    // 3. test creating an obsidian robot at this stage
    if state.ore >= blueprint.obsidian.0 && state.clay >= blueprint.obsidian.1 {
        let new_state = State {
            ore: ore - blueprint.obsidian.0,
            clay: clay - blueprint.obsidian.1,
            obsidian,
            geodes,
            ttl,
            obsidian_robots: state.obsidian_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, blueprint, cache));
    }

    // 4. test creating a geode robot at this stage
    if state.ore >= blueprint.geode.0 && state.obsidian >= blueprint.geode.1 {
        let new_state = State {
            ore: ore - blueprint.geode.0,
            clay,
            obsidian: obsidian - blueprint.geode.1,
            geodes,
            ttl,
            geode_robots: state.geode_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, blueprint, cache));
    }

    // 5. test not building any robot, but still harvesting resources!
    let do_nothing_state = State {
        ore,
        clay,
        obsidian,
        geodes,
        ttl,
        ..state
    };
    best = max(best, best_value(do_nothing_state, blueprint, cache));

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
    return state.geodes + (state.geode_robots * state.ttl) + ((state.ttl * (state.ttl - 1)) / 2);
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
            ore,
            clay,
            obsidian: (obs_1, obs_2),
            geode: (geode_1, geode_2),
        },
    )
}
