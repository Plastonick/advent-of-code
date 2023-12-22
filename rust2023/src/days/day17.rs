use crate::common::{get_lines, Answer};
use crate::maps::Vector;
use crate::Args;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Particle {
    position: Vector,
    direction: Vector,
    count: i8,
    value: usize,
    path: Vec<Vector>,
}

struct Map {
    tiles: HashMap<Vector, usize>,
    size: Vector,
}

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day17-test")
    } else {
        get_lines("day17")
    };

    let tiles = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| {
                    let val: usize = ch.to_string().parse().unwrap();

                    (
                        Vector {
                            row: row as isize,
                            col: col as isize,
                        },
                        val,
                    )
                })
                .collect::<Vec<(Vector, usize)>>()
        })
        .flatten()
        .collect::<HashMap<Vector, usize>>();

    let (row, col) = tiles
        .iter()
        .map(|(a, _)| (a.row, a.col))
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();
    let map = Map {
        tiles,
        size: Vector { row, col },
    };

    // start our wave off at the origin, going down and right
    let mut wave: Vec<Particle> = vec![
        Particle {
            position: Vector { row: 0, col: 0 },
            direction: Vector { row: 0, col: 1 },
            count: 1,
            value: 0,
            path: vec![],
        },
        Particle {
            position: Vector { row: 0, col: 0 },
            direction: Vector { row: 1, col: 0 },
            count: 1,
            value: 0,
            path: vec![],
        },
    ];
    // let best_final = depth_first_search(&wave, &map.size, &map);
    let best_final = best_path(&wave, &map);

    (best_final.to_string(), "".to_string())
}

fn depth_first_search(init: &Vec<Particle>, target: &Vector, map: &Map) -> usize {
    let mut cache_mutex: Arc<Mutex<HashMap<(Vector, Vector, i8), usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut wave: BTreeMap<isize, Particle> = BTreeMap::new();
    wave.insert(0, init[0].clone());
    wave.insert(0, init[1].clone());

    while let Some((_, particle)) = wave.pop_first() {
        let possibilities = get_possibilities(particle, &map);

        for particle in possibilities {
            let key = (particle.position, particle.direction, particle.count);

            let fudge = particle.position.row + (particle.position.col * map.size.row);
            let sort_value = (particle.value as isize * (map.size.row * map.size.col)) + fudge;
            let cache_mutex = Arc::clone(&cache_mutex);

            let mut cache = cache_mutex.lock().unwrap();
            if let Some(existing) = cache.get(&key) {
                // we have an existing route here, is it better?
                if &particle.value < existing {
                    if &particle.position == target {
                        println!("{}", particle.value);
                    }
                    // it's better, let's use it!
                    cache.insert(key, particle.value);
                    wave.insert(sort_value, particle);
                } else {
                    // it's not better, do nothing
                }
            } else {
                // we don't have an existing route here, this is naively the best
                cache.insert(key, particle.value);
                wave.insert(sort_value, particle);
            }
        }
    }

    let x = *cache_mutex
        .lock()
        .unwrap()
        .iter()
        .filter_map(|((pos, _, _), value)| if pos == target { Some(value) } else { None })
        .reduce(|a, b| a)
        .unwrap();

    x
}

fn get_directions(particle: &Particle) -> Vec<(Vector, i8)> {
    let mut valid_directions = vec![
        // go left
        (
            Vector {
                row: -particle.direction.col,
                col: particle.direction.row,
            },
            1,
        ),
        // go right
        (
            Vector {
                row: particle.direction.col,
                col: -particle.direction.row,
            },
            1,
        ),
    ];

    if particle.count < 3 {
        // go straight on, if allowed
        valid_directions.push((particle.direction, particle.count + 1));
    }

    valid_directions
}

fn get_possibilities(particle: Particle, map: &Map) -> Vec<Particle> {
    get_directions(&particle)
        .into_iter()
        .filter_map(|(direction, count)| {
            let new_pos = particle.position.add(&direction);

            if let Some(new_cost) = map.tiles.get(&new_pos) {
                let mut new_path = particle.path.clone();
                new_path.push(new_pos);

                Some(Particle {
                    position: new_pos,
                    direction,
                    count,
                    value: particle.value + new_cost,
                    path: new_path,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

// fn filter_possibilities() {
//
//     .filter_map(|next_particle| {
//         if let Some(&existing) = best.get(&(next_particle.position, next_particle.count)) {
//             // we already have a value to this point, is it better?
//             if next_particle.value < existing {
//                 // it's better! Add it, and continue the wave
//
//                 if visualise && next_particle.position == map.size {
//                     _print_path(&next_particle, &map);
//                 }
//                 if next_particle.position == map.size {
//                     println!("{}", next_particle.value);
//                 }
//
//                 best.insert(
//                     (next_particle.position, next_particle.count),
//                     next_particle.value,
//                 );
//                 Some(next_particle)
//             } else {
//                 // it's worse... do nothing
//                 None
//             }
//         } else {
//             // we've not seen this point before, insert this path as the best we've seen so far
//             best.insert(
//                 (next_particle.position, next_particle.count),
//                 next_particle.value,
//             );
//
//             Some(next_particle)
//         }
//     })
// }

fn best_path(wave: &Vec<Particle>, map: &Map) -> usize {
    let mut best: HashMap<(Vector, i8), usize> = HashMap::new();
    let mut wave = wave.clone();

    while wave.len() > 0 {
        println!("{}", wave.len());

        let (new_best, next_wave) = wave
            .clone()
            .into_iter()
            .map(|particle| get_possibilities(particle, &map))
            .flatten()
            .fold(
                (HashMap::new(), Vec::new()),
                |(mut best, mut wave), next_particle| {
                    if let Some(&existing) =
                        best.get(&(next_particle.position, next_particle.count))
                    {
                        // we already have a value to this point, is it better?
                        if next_particle.value < existing {
                            // it's better! Add it, and continue the wave

                            if next_particle.position == map.size {
                                println!("{}", next_particle.value);
                            }

                            best.insert(
                                (next_particle.position, next_particle.count),
                                next_particle.value,
                            );
                            wave.push(next_particle);
                        } else {
                            // it's worse... do nothing
                        }
                    } else {
                        // we've not seen this point before, insert this path as the best we've seen so far
                        best.insert(
                            (next_particle.position, next_particle.count),
                            next_particle.value,
                        );

                        wave.push(next_particle);
                    }

                    (best, wave)
                },
            );

        for (a, new_value) in new_best {
            if let Some(value) = best.get(&a) {
                if &new_value < value {
                    best.insert(a, new_value);
                }
            }
        }

        wave = next_wave;
    }

    *best
        .iter()
        .filter_map(|((pos, _), value)| if pos == &map.size { Some(value) } else { None })
        .reduce(|a, b| a)
        .unwrap()
}

fn _print_path(particle: &Particle, map: &Map) {
    let path_points: HashSet<Vector> = HashSet::from_iter(particle.path.clone().into_iter());

    (0..20).into_iter().for_each(|_| println!());

    let map_string = (0..=map.size.row)
        .into_iter()
        .map(|row| {
            (0..=map.size.col)
                .into_iter()
                .map(|col| {
                    let pos = Vector { row, col };

                    if let Some(_) = path_points.get(&pos) {
                        ".".to_owned()
                    } else if let Some(tile) = map.tiles.get(&pos) {
                        tile.to_string()
                    } else {
                        panic!("Uh oh!");
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{map_string}\n{}\n", particle.value);

    sleep(Duration::new(0, 10_000_000));
}
