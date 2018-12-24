use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use self::regex::Regex;

#[derive(Debug)]
struct Star {
    x: i64,
    y: i64,
    x_vel: i64,
    y_vel: i64
}

impl Star {
    fn new(x: i64, y: i64, x_vel: i64, y_vel: i64) -> Star {
        Star {
            x, y, x_vel, y_vel
        }
    }

    fn increment(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}

fn print_stars(pt_set: &HashSet<(i64, i64)>, left: i64, right: i64, top: i64, bottom: i64) {
    for y in top..bottom+1 {
        let star_map: String = (left..right+1).map(|x| match pt_set.contains(&(x, y)) {
            true => "#",
            false => ".",
        }).collect();
        println!("{}", star_map);
    }
}

pub fn q1(fname: String) -> String {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let star_re = Regex::new(r"position=<\s?(\S+),\s+(\S+)> velocity=<\s?(\S+),\s+(\S+)>").unwrap();
    let mut star_list: Vec<Star> = f_str.lines().map(|s| {
        let caps = star_re.captures(&s).unwrap();
        Star::new(
            caps.get(1).unwrap().as_str().parse().expect("string is empty"),
            caps.get(2).unwrap().as_str().parse().expect("string is empty"),
            caps.get(3).unwrap().as_str().parse().expect("string is empty"),
            caps.get(4).unwrap().as_str().parse().expect("string is empty"),
        )
    }).collect();

    let mut min_bounding_area: i64 = std::i64::MAX;
    let mut t = 0;
    let message_t = 10641;
    loop {
        let mut left_boundary: i64 = std::i64::MAX;
        let mut top_boundary: i64 = std::i64::MAX;
        let mut right_boundary: i64 = std::i64::MIN;
        let mut bottom_boundary: i64 = std::i64::MIN;
        t += 1;
        for star in star_list.iter_mut() {
            star.increment();
            if star.x < left_boundary {
                left_boundary = star.x;
            }
            if star.x > right_boundary {
                right_boundary = star.x;
            }
            if star.y < top_boundary {
                top_boundary = star.y;
            }
            if star.y > bottom_boundary {
                bottom_boundary = star.y;
            }
        }
        let box_area = (right_boundary - left_boundary) * (bottom_boundary - top_boundary);
        if box_area < min_bounding_area && t != message_t {
            min_bounding_area = box_area;
        } else {
            let pt_set: HashSet<(i64, i64)> = star_list.iter().map(|star| (star.x, star.y)).collect();
            println!("t = {}", t);
            print_stars(&pt_set, left_boundary, right_boundary, top_boundary, bottom_boundary);
            break;
        }
    }

    String::new()
}

