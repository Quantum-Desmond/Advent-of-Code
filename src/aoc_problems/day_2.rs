use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

fn contains_letter_count(s: &String, n: i32) -> bool {
    let mut char_count: HashMap<_, _> = HashMap::new();
    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    char_count.values().any(|&x| x == n)
}

fn str_diff(ref s1: &String, ref s2: &String) -> i32 {
    s1.chars().zip(s2.chars()).map(|(c1, c2)| {
        (c1 != c2) as i32
    }).sum()
}

pub fn q1(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let id_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();


    let id_scores: Vec<(i32, i32)> = id_list.iter().map(|ref s: &String| {
        (contains_letter_count(&s, 2) as i32, contains_letter_count(&s, 3) as i32)
    }).collect();

    let double: i32 = id_scores.iter().map(|&x| {
        x.0
    }).sum();
    let triple: i32 = id_scores.iter().map(|&x| {
        x.1
    }).sum();

    double * triple
}

pub fn q2(fname: String) -> String {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let id_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let mut diff_count: HashMap<_, _> = HashMap::new();

    for (idx, id1) in id_list.iter().enumerate() {
        for id2 in id_list[idx+1..].iter() {
            diff_count.insert(
                (id1, id2),
                str_diff(&id1, &id2)
            );
        }
    }

    let (s1, s2) = diff_count.iter().find(|(&_k, &v)| v == 1).unwrap().0;

    s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _c2)| c1).collect()
}
