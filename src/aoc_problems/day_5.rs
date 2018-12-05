use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

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

fn is_relevant_pair(c1: char, c2: char) -> bool {
    (c1.to_lowercase().to_string() == c2.to_lowercase().to_string())
        && (c1.is_lowercase() ^ c2.is_lowercase())
}

fn clean_up_polymer(char_list: &Vec<char>)  -> Vec<char> {
    let mut new_char_list: Vec<_> = Vec::new();

    let mut removed_chars: Vec<_> = Vec::new();

    let mut idx: usize = 0;
    loop {
        let c1 = char_list[idx];
        let c2 = char_list[idx+1];

        if is_relevant_pair(c1, c2) {
            removed_chars.push(c1);
            removed_chars.push(c2);
            idx += 2;
        } else {
            new_char_list.push(c1);
            idx += 1;
        }

        if idx >= char_list.len()-1 {
            if idx == char_list.len()-1 {
                new_char_list.push(char_list[idx]);
            }
            break;
        }
    }

    if char_list.len() != new_char_list.len() {
        if cfg!(debug_assertions) {
            println!("Next length = {}", new_char_list.len());
            println!("Removed: {:?}", removed_chars);
        }
        clean_up_polymer(&new_char_list)
    } else {
        char_list.to_vec()
    }

}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");

    let mut polymer_str = String::new();
    f.read_to_string(&mut polymer_str).expect("Couldn't find file");
    let polymer_str = polymer_str.trim();

    let char_list: Vec<_> = polymer_str.chars().collect();
    if cfg!(debug_assertions) {
        println!("First length = {}", char_list.len());
    }
    let char_list = clean_up_polymer(&char_list);

    char_list.len()
}

pub fn q2(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");

    let mut polymer_str = String::new();
    f.read_to_string(&mut polymer_str).expect("Couldn't find file");
    let polymer_str = polymer_str.trim();

    let char_list: Vec<_> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i",
        "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z"
    ];
    let mut scores: HashMap<_, _> = HashMap::new();
    for c in char_list.iter() {
        let new_string = polymer_str.replace(c, "").replace(&c.to_uppercase(), "");

        let char_list: Vec<_> = new_string.chars().collect();
        let char_list = clean_up_polymer(&char_list);

        scores.insert(c, char_list.len());
    }

    *scores.values().min().unwrap()
}
