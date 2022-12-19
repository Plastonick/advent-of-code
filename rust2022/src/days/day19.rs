use std::{cmp::max, collections::HashMap};

use regex::Regex;

use crate::common::get_lines;

#[derive(Debug)]
struct Blueprint {
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

pub fn run(_: bool) {
    let lines = get_lines("day19-test");
    let blueprints = lines.iter().map(build_blueprint).collect::<HashMap<_, _>>();

    // what now?
    let ttl = 24;
    let mut quality_level_total = 0;

    for (index, blueprint) in blueprints {
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
        let value = best_value(start, &blueprint, &mut cache);

        println!("The quality level of blueprint #{} is {}", index, value);

        quality_level_total += value * index;
    }

    println!(
        "Day 19, Part 1: Quality level total is {}",
        quality_level_total
    );
}

fn best_value(state: State, blueprint: &Blueprint, cache: &mut HashMap<State, i32>) -> i32 {
    // if time is up, return the number of geodes we have
    if state.ttl == 0 {
        return state.geodes;
    }

    if let Some(value) = cache.get(&state) {
        return value.to_owned();
    }

    let mut best = 0;

    // permute possible actions and determine best
    // make sure to include a "do nothing" option to allow for saving resources for a more expensive robot

    // each robot will collect 1 piece of its resource
    let ore = state.ore + state.ore_robots;
    let clay = state.clay + state.clay_robots;
    let obsidian = state.obsidian + state.obsidian_robots;
    let geodes = state.geodes + state.geode_robots;
    let ttl = state.ttl - 1;

    // test creating an ore robot at this stage
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

        best = max(best, best_value(new_state, &blueprint, cache));
    }

    // test creating a clay robot at this stage
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

        best = max(best, best_value(new_state, &blueprint, cache));
    }

    // test creating an obsidian robot at this stage
    if state.ore >= blueprint.obsidian.0 && state.clay >= blueprint.obsidian.1 {
        let new_state = State {
            ore: state.ore - blueprint.obsidian.0,
            clay: state.clay - blueprint.obsidian.1,
            obsidian,
            geodes,
            ttl,
            obsidian_robots: state.obsidian_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, &blueprint, cache));
    }

    // test creating a geode robot at this stage
    if state.ore >= blueprint.geode.0 && state.obsidian >= blueprint.geode.1 {
        let new_state = State {
            ore: state.ore - blueprint.geode.0,
            clay,
            obsidian: state.obsidian - blueprint.geode.1,
            geodes,
            ttl,
            geode_robots: state.geode_robots + 1,
            ..state
        };

        best = max(best, best_value(new_state, &blueprint, cache));
    }

    // test not building any robot
    let do_nothing_state = State {
        ore,
        clay,
        obsidian,
        geodes,
        ttl,
        ..state
    };
    best = max(best, best_value(do_nothing_state, &blueprint, cache));

    // cache the best value we could make at this state
    cache.insert(state, best);

    best
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
            ore,
            clay,
            obsidian: (obs_1, obs_2),
            geode: (geode_1, geode_2),
        },
    )
}
