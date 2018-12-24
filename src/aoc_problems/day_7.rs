use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use self::regex::Regex;

#[derive(Debug)]
pub struct Order {
    prereq: char,
    next: char,
}

impl Order {
    fn new(prereq: char, next: char) -> Order {
        Order {prereq, next}
    }
}

#[derive(Debug, Clone)]
pub struct Booking {
    number: i32,
    current: Option<char>,
    finish_t: i32,
}

impl Booking {
    fn new(number: i32, current: Option<char>, finish_t: i32) -> Booking {
        Booking {number, current, finish_t}
    }
}

fn char_timing(c: char) -> i32 {
    (c.to_digit(36).unwrap() as i32) - 9 + 60
}


pub fn q1(fname: String) -> String {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let pattern_re = Regex::new(r"Step (\S) must be finished before step (\S) can begin.").unwrap();
    let prereq_list: Vec<_> = f_str.lines().map(|x: &str| {
        let caps = pattern_re.captures(&x).unwrap();
        Order::new(
            caps.get(1).unwrap().as_str().chars().next().expect("string is empty"),
            caps.get(2).unwrap().as_str().chars().next().expect("string is empty"),
        )
    }).collect();


    let char_list: Vec<_> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let mut step_prereqs: HashMap<char, Vec<char>> = HashMap::new();
    for c in char_list {
        step_prereqs.insert(c, vec![]);
    }

    for order in prereq_list {
        if let Some(v) = step_prereqs.get_mut(&order.next) {
            v.push(order.prereq);
        }
    }

    // first, get all with no prereqs
    // order alphabetically
    // do first one
    // repeat until all done

    let mut step_order: Vec<char> = Vec::new();
    while step_order.len() < 26 {
        let mut valid_steps: Vec<char> = step_prereqs.iter().filter(
            |(ref _k, ref v)| v.is_empty()
        ).map(|(ref k, ref _v)| **k).collect();
        valid_steps.sort();

        let c: char = valid_steps[0];
        step_order.push(c);

        step_prereqs.remove(&c);

        for prereqs in step_prereqs.values_mut() {
            if let Some(idx) = prereqs.iter().position(|&x| x == c) {
                prereqs.remove(idx);
            }
        }
    }

    step_order.iter().collect()
}

pub fn q2(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let pattern_re = Regex::new(r"Step (\S) must be finished before step (\S) can begin.").unwrap();
    let prereq_list: Vec<_> = f_str.lines().map(|x: &str| {
        let caps = pattern_re.captures(&x).unwrap();
        Order::new(
            caps.get(1).unwrap().as_str().chars().next().expect("string is empty"),
            caps.get(2).unwrap().as_str().chars().next().expect("string is empty"),
        )
    }).collect();


    // let char_list: Vec<_> = vec![
    //     'A', 'B', 'C', 'D', 'E', 'F'
    // ];
    let char_list: Vec<_> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let mut step_prereqs: HashMap<char, Vec<char>> = HashMap::new();
    for &c in char_list.iter() {
        step_prereqs.insert(c, vec![]);
    }

    // this is the list of all prereqs
    for order in prereq_list {
        if let Some(v) = step_prereqs.get_mut(&order.next) {
            v.push(order.prereq);
        }
    }

    let elf_num = 5;
    let mut elf_bookings: Vec<Booking> = (1..elf_num+1).map(|n| Booking::new(n, None, 0)).collect();
    let mut t: i32 = 0;
    let mut occupied_jobs: HashSet<char> = HashSet::new();
    let mut step_order: Vec<char> = vec![];
    loop {
        for booking in elf_bookings.iter_mut() {
            if booking.finish_t == t {
                if let Some(c) = booking.current {
                    step_order.push(c);
                    println!("Finished step '{}' at time {} by elf {}", c, t, booking.number);
                    if step_order.len() == char_list.len() {
                        println!("t = {}: Finished! Step order = {:?}", t, step_order);
                        return t+1;
                    }

                    // remove c from currently worked on
                    if !occupied_jobs.remove(&c) {
                        println!("Broken! Cannot find correct char in occupied list");
                        panic!();
                    }

                    // next, remove all of the instances of c in step_prereqs
                    if let Some(_v) = step_prereqs.remove(&c) {
                        println!("Removed key {} from step_prereqs", c);
                    }
                    for prereqs in step_prereqs.values_mut() {
                        if let Some(idx) = prereqs.iter().position(|&x| x == c) {
                            prereqs.remove(idx);
                        }
                    }

                    booking.current = None;
                }

                let mut valid_steps: Vec<char> = step_prereqs.iter().filter(
                    |(ref _k, ref v)| v.is_empty()
                ).map(|(ref k, ref _v)| **k).filter(
                    |k| !occupied_jobs.contains(&k)
                ).collect();
                valid_steps.sort();

                if !valid_steps.is_empty() {
                    let c: char = valid_steps[0];
                    booking.current = Some(c);
                    booking.finish_t = t + char_timing(c);
                    if !occupied_jobs.insert(c) {
                        println!("Duplicate added!");
                        panic!();
                    }
                    println!("{} is now being worked on by elf {}", c, booking.number);
                } else {
                    booking.finish_t += 1;
                }
            }
        }
        t += 1;
    }
}
