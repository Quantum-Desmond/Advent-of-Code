use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use self::regex::Regex;

extern crate chrono;
use self::chrono::prelude::*;

#[derive(Debug)]
struct GuardEvent {
    dt: DateTime<Utc>,
    event: String
}

impl GuardEvent {
    fn new(dt_str: String, event: String) -> GuardEvent {
        GuardEvent {
            dt: Utc.datetime_from_str(&dt_str, "%Y-%m-%d %H:%M").unwrap(),
            event
        }
    }
}

pub fn q1(fname: String) -> u32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let claim_str_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let event_re = Regex::new(r"\[(.+)\] (.+)$").unwrap();
    let mut claim_list: Vec<_> = claim_str_list.iter().map(|ref s| {
        let caps = event_re.captures(&s).unwrap();
        GuardEvent::new(
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str().to_string(),
        )
    }).collect();

    claim_list.sort_by_key(|claim| claim.dt);

    let guard_event_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut asleep_times: HashMap<_, _> = HashMap::new();

    let mut guard_id: u32 = 0;
    let mut start_minute: u32 = 0;
    for claim in claim_list.iter() {
        match &claim.event[..] {
            "falls asleep" => {
                start_minute = claim.dt.minute();
            },
            "wakes up" => {
                let minute_list = asleep_times.entry(guard_id).or_insert(vec![]);
                for t in start_minute..claim.dt.minute() {
                    minute_list.push(t);
                }
                start_minute = 0;
            },
            c if c.starts_with("Guard #") => {
                let caps = guard_event_re.captures(c).unwrap();
                guard_id = caps.get(1).unwrap().as_str().parse().unwrap();
            },
            _ => continue,
        }
    }

    let (sleepiest_guard, minute_list) = asleep_times.iter().max_by_key(|&v| v.1.len()).unwrap();

    let mut minute_count: HashMap<_, _> = HashMap::new();
    for minute in minute_list.iter() {
        let count = minute_count.entry(minute).or_insert(0);
        *count += 1;
    }

    let common_minute = minute_count.iter().max_by_key(|&(_k, v)| v).unwrap().0;

    *sleepiest_guard * **common_minute
}

pub fn q2(fname: String) -> u32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let claim_str_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let event_re = Regex::new(r"\[(.+)\] (.+)$").unwrap();
    let mut claim_list: Vec<_> = claim_str_list.iter().map(|ref s| {
        let caps = event_re.captures(&s).unwrap();
        GuardEvent::new(
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str().to_string(),
        )
    }).collect();

    claim_list.sort_by_key(|claim| claim.dt);

    let guard_event_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut asleep_times: HashMap<_, _> = HashMap::new();

    let mut guard_id: u32 = 0;
    let mut start_minute: u32 = 0;
    for claim in claim_list.iter() {
        match &claim.event[..] {
            "falls asleep" => {
                start_minute = claim.dt.minute();
            },
            "wakes up" => {
                let minute_list = asleep_times.entry(guard_id).or_insert(vec![]);
                for t in start_minute..claim.dt.minute() {
                    minute_list.push(t);
                }
                start_minute = 0;
            },
            c if c.starts_with("Guard #") => {
                let caps = guard_event_re.captures(c).unwrap();
                guard_id = caps.get(1).unwrap().as_str().parse().unwrap();
            },
            _ => continue,
        }
    }

    let most_common_minute: HashMap<_, _> = asleep_times.iter()
        .map(|(guard_id, ref v)| {
            let mut different_minutes: HashMap<_, _> = HashMap::new();
            for t in v.iter() {
                let time = different_minutes.entry(t).or_insert(0);
                *time += 1;
            }

            let (common_minute, minute_count) = different_minutes.iter().max_by_key(
                |&(_min, count)| count
            ).unwrap();

            (guard_id, (**common_minute, *minute_count))
        }).collect();

    let (guard_id, common_minute) = most_common_minute.iter().max_by_key(
        |&(_id, (_min, count))| count
    ).unwrap();

    **guard_id * common_minute.0
}
