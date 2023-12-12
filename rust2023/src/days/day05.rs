use crate::common::get_file_contents;
use crate::Args;

#[derive(Debug)]
struct Translation {
    source: (i128, i128),
    dest: (i128, i128),
}

impl Translation {
    fn factor(&self) -> i128 {
        self.dest.0 - self.source.0
    }
}

type Map = Vec<Translation>;
type Range = (i128, i128);

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

    let mut seed_ranges = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect::<Vec<(i128, i128)>>();

    for map in maps {
        seed_ranges = map_ranges(&seed_ranges, &map);
    }

    let lowest_location_of_ranges = seed_ranges.iter().map(|(l, _)| l).min().unwrap();

    if !args.no_answers {
        println!("Day 5, Part 1: The lowest location is {lowest_location}");
        println!("Day 5, Part 2: The lowest location of the ranges is {lowest_location_of_ranges}");
    }

    (
        lowest_location.to_string(),
        lowest_location_of_ranges.to_string(),
    )
}

fn map_to_location(source: i128, maps: &Vec<Map>) -> i128 {
    let mut source = source;

    for map in maps {
        source = map_source(source, &map);
    }

    source
}

fn map_source(source: i128, map: &Map) -> i128 {
    let ranges = map
        .iter()
        .filter(|x| source >= x.source.0 && source < x.source.1)
        .collect::<Vec<&Translation>>();

    match ranges.first() {
        None => source,
        Some(&range) => source + (range.dest.0 - range.source.0),
    }
}

fn map_ranges(ranges: &Vec<(i128, i128)>, map: &Map) -> Vec<(i128, i128)> {
    ranges
        .iter()
        .map(|range| map_range(range, map))
        .flatten()
        .collect::<Vec<_>>()
}

fn map_range(range: &(i128, i128), map: &Map) -> Vec<(i128, i128)> {
    let mut unmapped_ranges = vec![(range.0, range.1)];
    let mut mapped_ranges = vec![];

    for translation in map {
        if unmapped_ranges.len() == 0 {
            break;
        }

        let mut loop_unmatched = vec![];

        for range in unmapped_ranges {
            let (mapped, unmapped) = translate_range(range, &translation);

            if let Some(mapped_range) = mapped {
                mapped_ranges.push(mapped_range);
            }
            loop_unmatched.append(&mut unmapped.clone());
        }

        unmapped_ranges = loop_unmatched;
    }

    mapped_ranges.append(&mut unmapped_ranges);

    mapped_ranges
}

// returns tuple of mapped range, and unmapped range(s)
fn translate_range(seeds: Range, translation: &Translation) -> (Option<Range>, Vec<Range>) {
    let intersect_min = seeds.0.max(translation.source.0);
    let intersect_max = seeds.1.min(translation.source.1);

    // there's no intersection, so trivially return the seeds
    if intersect_min > intersect_max {
        return (None, vec![(seeds.0, seeds.1)]);
    }

    // some intersection; three "output" ranges, some of which will be "empty" and so can be ignored
    // 1. [seed min, intersect min)
    // 2. [intersect min, intersect max]
    // 3. (intersect max, seed max]
    let mut unmapped = Vec::new();
    let mut mapped = None;

    if seeds.0 < intersect_min {
        unmapped.push((seeds.0, intersect_min - 1));
    }

    if intersect_min < intersect_max {
        mapped = Some((
            intersect_min + translation.factor(),
            intersect_max + translation.factor(),
        ));
    }

    if intersect_max < seeds.1 {
        unmapped.push((intersect_max + 1, seeds.1));
    }

    (mapped, unmapped)
}

fn build_maps(file: String) -> (Vec<i128>, Vec<Map>) {
    let (seeds_string, maps_string) = file.split_once('\n').unwrap();

    let seed_numbers: Vec<i128> = seeds_string
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
                        .filter_map(|n| n.parse::<i128>().ok())
                        .take(3)
                        .collect::<Vec<i128>>();

                    let dest_start = *numbers.get(0).unwrap();
                    let source_start = *numbers.get(1).unwrap();
                    let length = *numbers.get(2).unwrap();

                    Translation {
                        source: (source_start, source_start + (length - 1)),
                        dest: (dest_start, dest_start + (length - 1)),
                    }
                })
                .collect::<Vec<Translation>>()
        })
        .collect::<Vec<Vec<Translation>>>();

    (seed_numbers, maps)
}

#[test]
fn test_intersections() {
    for (seeds, translation, expected) in _range_samples() {
        assert_eq!(expected, translate_range(seeds, &translation));
    }
}

fn _range_samples() -> Vec<(Range, Translation, (Option<Range>, Vec<Range>))> {
    vec![
        (
            (3, 6),
            Translation {
                source: (1, 2),
                dest: (21, 22),
            },
            (None, vec![(3, 6)]),
        ),
        (
            (3, 6),
            Translation {
                source: (1, 4),
                dest: (21, 24),
            },
            (Some((23, 24)), vec![(5, 6)]),
        ),
        (
            (3, 6),
            Translation {
                source: (4, 5),
                dest: (24, 25),
            },
            (Some((24, 25)), vec![(3, 3), (6, 6)]),
        ),
        (
            (3, 6),
            Translation {
                source: (1, 7),
                dest: (21, 27),
            },
            (Some((23, 26)), vec![]),
        ),
        (
            (3, 6),
            Translation {
                source: (5, 7),
                dest: (25, 27),
            },
            (Some((25, 26)), vec![(3, 4)]),
        ),
        (
            (3, 6),
            Translation {
                source: (7, 8),
                dest: (27, 28),
            },
            (None, vec![(3, 6)]),
        ),
    ]
}
