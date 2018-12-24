use std::collections::VecDeque;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use self::regex::Regex;

fn char_to_bool(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => {
            println!("Unexpected char!");
            panic!();
        }
    }
}

fn get_char_from_deque(deque: &VecDeque<char>, n: i32) -> char {
    match n {
        x if x < 0 => {
            '.'
        },
        _ => {
            *deque.get(n as usize).unwrap_or(&'.')
        }
    }
}

fn get_char_from_deque2(deque: &VecDeque<char>, n: i64) -> char {
    match n {
        x if x < 0 => {
            '.'
        },
        _ => {
            *deque.get(n as usize).unwrap_or(&'.')
        }
    }
}

pub fn q1(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    let initial_state_re = Regex::new(r"initial state: (\S+)").unwrap();
    let transform_re = Regex::new(r"(\S+) => (\S)").unwrap();
    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let input_list: Vec<String> = f_contents.lines().map(|s| s.to_string()).collect();

    let init_state_cap = initial_state_re.captures(&input_list[0]).unwrap();

    let mut pot_states: VecDeque<char> = init_state_cap.get(1).unwrap().as_str().chars().collect();
    let mut first_idx: i32 = 0;

    let pot_growth: HashMap<&str, char> = input_list[2..].iter().map(|s| {
        let cap = transform_re.captures(&s).unwrap();
        let start = cap.get(1).unwrap().as_str();
        let end: char = cap.get(2).unwrap().as_str().chars().next().unwrap();

        (start, end)
    }).collect();

    for _ in 0..20 {
        let mut new_state: VecDeque<char> = VecDeque::new();
        // let list_length = pot_states.len();
        for idx in -2..((pot_states.len()+2) as i32) {
            let s: String = (idx-2..idx+3).map(|n| get_char_from_deque(&pot_states, n)).collect();
            let new_c: char = pot_growth[&s[..]];
            if idx < 0 && new_c == '#' {
                new_state.push_back(new_c);
                first_idx -= 1;
            } else if idx >= 0 {
                new_state.push_back(new_c);
            }
        }
        pot_states.clear();
        pot_states.append(&mut new_state);
    }

    pot_states.iter().enumerate().filter(|(_n, c)| **c == '#').map(|(n, _c)| n as i32 + first_idx).sum()

}

