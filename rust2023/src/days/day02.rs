use crate::common::get_lines;
use crate::Args;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(input: &str) -> Result<Colour, Self::Err> {
        match input {
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            _ => Err(()),
        }
    }
}

type Show = HashMap<Colour, usize>;
type Game = (isize, Vec<Show>);

pub fn run(args: &Args) -> (String, String) {
    let lines = get_lines("day02");
    let limit = HashMap::from([(Colour::Red, 12), (Colour::Green, 13), (Colour::Blue, 14)]);

    let valid_game_id_sum: isize = lines
        .iter()
        .map(build_game_shows)
        .filter(|(_, shows)| valid_game(&limit, shows))
        .map(|(game_id, _)| game_id)
        .sum();

    let power_sum: usize = lines
        .iter()
        .map(build_game_shows)
        .map(|(_, shows)| get_best(shows))
        .map(calculate_power)
        .sum::<usize>();

    (valid_game_id_sum.to_string(), power_sum.to_string())
}

fn calculate_power(show: Show) -> usize {
    show.get(&Colour::Red).unwrap()
        * show.get(&Colour::Green).unwrap()
        * show.get(&Colour::Blue).unwrap()
}

fn get_best(shows: Vec<Show>) -> Show {
    let mut best_show = HashMap::new();

    for colour in [Colour::Red, Colour::Blue, Colour::Green] {
        let best_count = shows
            .iter()
            .map(|x| x.get(&colour).unwrap_or(&0))
            .max()
            .unwrap();

        best_show.insert(colour, *best_count);
    }

    best_show
}

fn build_game_shows(line: &String) -> Game {
    let game_pattern: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();

    let captures = game_pattern.captures(line).unwrap();
    let game_id = captures
        .get(1)
        .map(|x| x.as_str().parse::<isize>().unwrap())
        .unwrap();
    let shows = captures
        .get(2)
        .map(|x| x.as_str())
        .unwrap()
        .split("; ")
        .map(|x| parse_show(x))
        .collect::<Vec<_>>();

    (game_id, shows)
}

fn parse_show(line: &str) -> Show {
    line.split(", ")
        .map(|x| x.split_once(' ').unwrap())
        .map(|(count_str, colour_str)| {
            let count = count_str.parse::<usize>().unwrap();
            let colour = Colour::from_str(colour_str).unwrap();

            (colour, count)
        })
        .collect::<Show>()
}

fn valid_game(limit: &Show, shows: &Vec<Show>) -> bool {
    for show in shows {
        for (colour, count) in show {
            if count > limit.get(colour).unwrap() {
                return false;
            }
        }
    }

    true
}

// fn get_counts(show: Vec<&str>) -> Vec<(isize, &str)> {
//     let balls = show.split(", ").collect::<Vec<_>>();
//
//     balls
//         .iter()
//         .map(|&x| {
//             let (count, colour) = x.split_once(' ').unwrap();
//             let count_int = count.parse::<isize>().unwrap();
//
//             (count_int, colour)
//         })
//         .collect::<Vec<_>>()
// }
