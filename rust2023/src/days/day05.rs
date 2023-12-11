use crate::common::get_file_contents;
use crate::Args;

#[derive(Debug)]
struct Range {
    source: isize,
    dest: isize,
    range_length: isize,
}
type Map = Vec<Range>;

pub fn run(args: &Args) -> (String, String) {
    let input = if args.test {
        get_file_contents("day05-test")
    } else {
        get_file_contents("day05")
    };

    let (seeds, maps) = build_maps(input);

    let lowest_location = seeds
        .iter()
        .map(|s| map_to_location(*s, &maps))
        .min()
        .unwrap();

    // dbg!(seeds);
    // dbg!(maps.get(2));

    if !args.no_answers {
        println!("Day 5, Part 1: The lowest location is {lowest_location}");
        println!("Day 5, Part 2: TODO");
    }

    (lowest_location.to_string(), "".to_string())
}

fn map_to_location(source: isize, maps: &Vec<Map>) -> isize {
    let mut source = source;

    for map in maps {
        source = map_source(source, &map);
    }

    source
}

fn map_source(source: isize, map: &Map) -> isize {
    let ranges = map
        .iter()
        .filter(|x| source >= x.source && source < x.source + x.range_length)
        .collect::<Vec<&Range>>();

    match ranges.first() {
        None => source,
        Some(&range) => source + (range.dest - range.source),
    }
}

fn build_maps(file: String) -> (Vec<isize>, Vec<Map>) {
    let (seeds_string, maps_string) = file.split_once('\n').unwrap();

    let seed_numbers: Vec<isize> = seeds_string
        .split_at(7)
        .1
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let maps = maps_string
        .split("\n\n")
        .map(|x| {
            x.trim()
                .split_once("\n")
                .unwrap()
                .1
                .split("\n")
                .skip(0)
                .filter(|x| x.len() >= 1)
                .map(|x| {
                    let numbers = x
                        .split(" ")
                        .filter_map(|n| n.parse::<isize>().ok())
                        .take(3)
                        .collect::<Vec<isize>>();

                    let destination_range_start = *numbers.get(0).unwrap();
                    let source_range_start = *numbers.get(1).unwrap();
                    let range_length = *numbers.get(2).unwrap();

                    Range {
                        dest: destination_range_start,
                        source: source_range_start,
                        range_length,
                    }
                })
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<Vec<Range>>>();

    (seed_numbers, maps)
}
