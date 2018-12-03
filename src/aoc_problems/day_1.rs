use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

pub fn q1(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let increments: Vec<_> = f_contents.lines().collect();

    let total: i32 = increments.iter().map(|&x: &&str| {
        x.to_string().parse::<i32>().unwrap()
    }).sum();

    total
}

pub fn q2(fname: String) -> i32 {
    let mut result: i32 = 0;

    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let increments: Vec<_> = f_contents.lines().collect();
    let increments: Vec<i32> = increments.iter().map(|&x: &&str| {
        x.to_string().parse().unwrap()
    }).collect();

    let mut freqs = HashSet::new();

    let length = increments.len();
    let mut idx: usize = 0;
    loop {
        result += increments[idx];

        if freqs.contains(&result) {
            break;
        }

        freqs.insert(result);

        idx = (idx+1) % length;
    }

    result
}
