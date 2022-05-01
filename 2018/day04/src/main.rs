use chrono::{prelude::*, Duration};
use regex::{CaptureMatches, Captures, Regex};
use std::{collections::HashMap, fs, ops::Add, time};

struct Shift<'a> {
    guard_id: i32,
    events: Vec<SleepEvent<'a>>,
}

struct GuardChange {
    time: DateTime<Utc>,
    id: i32,
}

enum StateChange {
    Sleep,
    Wake,
}

struct SleepEvent<'a> {
    time: DateTime<Utc>,
    state: &'a StateChange,
}

struct Event<'a> {
    time: DateTime<Utc>,
    action: &'a str,
}

fn main() {
    let lines = read_lines();

    part1(&lines);
}

fn part1(lines: &Vec<&&mut Shift<'static>>) {
    let mut sleep_times: HashMap<i32, i64> = HashMap::new();
    let mut guard_ids = Vec::new();

    for line in lines {
        let chunks = line.events.chunks(2);
        let time_asleep = chunks
            .map(|x| x[1].time.timestamp() - x[0].time.timestamp())
            .reduce(|x, y| x + y)
            .unwrap_or(0);

        guard_ids.push(line.guard_id);
        sleep_times.insert(line.guard_id, time_asleep);
    }

    guard_ids.sort_by(|x, y| sleep_times[x].cmp(&sleep_times[y]));
    let sleepy_guard_id = guard_ids.last().unwrap();
    let sleepy_shift = lines
        .into_iter()
        .filter(|x| &x.guard_id == sleepy_guard_id)
        .last()
        .unwrap();

    let mut minute_sleep: HashMap<u32, i32> = HashMap::new();

    for events in sleepy_shift.events.chunks(2) {
        let event1 = &events[0];
        let event2 = &events[1];

        let mut start = event1.time.clone();

        while start < event2.time {
            if minute_sleep.contains_key(&start.minute()) {
                minute_sleep.insert(
                    start.minute(),
                    minute_sleep.get(&start.minute()).unwrap() + 1,
                );
            } else {
                minute_sleep.insert(start.minute(), 1);
            }

            println!("{}", start.minute());

            start = start + Duration::minutes(1);
        }
    }

    println!("guard #{} slept the most", sleepy_guard_id);
}

fn read_lines() -> Vec<&'static &'static mut Shift<'static>> {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    // [1518-11-01 00:00] Guard #10 begins shift
    let re = Regex::new(r".(\d+)-(\d+)-(\d+) (\d+):(\d+). (.*)").unwrap();
    let lines = contents.lines().into_iter();
    let mut events: Vec<Event> = Vec::new();

    for line in lines {
        let captures = re.captures(line).unwrap();

        let year = captures
            .get(1)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .unwrap();
        let month = captures
            .get(2)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .unwrap();
        let day = captures
            .get(3)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .unwrap();
        let hours = captures
            .get(4)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .unwrap();
        let minutes = captures
            .get(5)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .unwrap();

        let action = captures.get(6).map(|m| m.as_str()).unwrap();

        let dt = Utc.ymd(year, month, day).and_hms(hours, minutes, 0);

        events.push(Event {
            time: dt,
            action: action,
        });
    }

    events.sort_by(|a, b| a.time.timestamp().cmp(&b.time.timestamp()));

    let mut shifts: HashMap<i32, &mut Shift> = HashMap::new();
    let mut current = &mut Shift {
        guard_id: 0,
        events: Vec::new(),
    };

    let action_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut sleep_event = &StateChange::Sleep;

    for event in events {
        if action_re.is_match(event.action) {
            let caps = action_re.captures(event.action).unwrap();

            let guard_id = caps
                .get(1)
                .map_or(1, |m| m.as_str().parse::<i32>().unwrap());

            current = if shifts.contains_key(&guard_id) {
                *shifts.get(&guard_id).unwrap()
            } else {
                &mut Shift {
                    guard_id: guard_id,
                    events: Vec::new(),
                }
            };
        } else {
            current.events.push(SleepEvent {
                time: event.time,
                state: &sleep_event,
            });

            sleep_event = if matches!(sleep_event, StateChange::Sleep) {
                &StateChange::Wake
            } else {
                &StateChange::Sleep
            };
        }
    }

    let mut shift_vec = Vec::new();
    for shift in shifts.values() {
        shift_vec.push(shift.clone());
    }

    shift_vec
}
