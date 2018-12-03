use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use self::regex::Regex;

#[derive(Debug)]
struct Claim {
    id: u32,
    tl_x: u32,
    tl_y: u32,
    width: u32,
    height: u32
}

impl Claim {
    fn new(id: u32, tl_x: u32, tl_y: u32, width: u32, height: u32) -> Claim {
        Claim {
            id,
            tl_x,
            tl_y,
            width,
            height
        }
    }
}

pub fn q1(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let claim_str_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let claim_re = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    let claim_list: Vec<Claim> = claim_str_list.iter().map(|ref s| {
        let caps = claim_re.captures(&s).unwrap();
        Claim::new(
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
            caps.get(5).unwrap().as_str().parse().unwrap(),
        )
    }).collect();

    let mut grid_map: HashMap<_, _> = HashMap::new();
    for claim in claim_list.iter() {
        for x in claim.tl_x..claim.tl_x+claim.width {
            for y in claim.tl_y..claim.tl_y+claim.height {
                let count = grid_map.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }

    grid_map.iter().filter(|(&_k, &v)| v > 1).count() as i32
}

pub fn q2(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let claim_str_list: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let claim_re = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    let claim_list: Vec<Claim> = claim_str_list.iter().map(|ref s| {
        let caps = claim_re.captures(&s).unwrap();
        Claim::new(
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
            caps.get(5).unwrap().as_str().parse().unwrap(),
        )
    }).collect();

    let mut valid_claims: HashMap<_, _> = claim_list.iter().map(
        |ref claim| (claim.id, true)
    ).collect();

    let mut grid_map: HashMap<_, _> = HashMap::new();
    for claim in claim_list.iter() {
        for x in claim.tl_x..claim.tl_x+claim.width {
            for y in claim.tl_y..claim.tl_y+claim.height {
                if let Some(id) = grid_map.insert((x, y), claim.id) {
                    valid_claims.insert(id, false);
                    valid_claims.insert(claim.id, false);
                }
            }
        }
    }

    *valid_claims.iter().find(|(&_k, &v)| v).unwrap().0 as i32
}
