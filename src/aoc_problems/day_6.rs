use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    fn dist_from(&self, x: i32, y: i32) -> i32 {
        // Manhattan distance
        (self.x - x).abs() + (self.y - y).abs()
    }
}

pub fn q1(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let point_list: Vec<_> = f_str.lines().map(|x: &str| {
        let split_str: Vec<i32> = x.split(", ").map(|n_str: &str| {
            n_str.parse().unwrap()
        }).collect();
        Point::new(split_str[0], split_str[1])
    }).collect();

    let mut point_areas: HashMap<_, _> = HashMap::new();
    let max_x: i32 = point_list.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y: i32 = point_list.iter().max_by_key(|p| p.y).unwrap().y;

    for x in 0..max_x+1 { for y in 0..max_y+1 {
        let mut closest_pts: Vec<usize> = Vec::new();
        let mut min_dist: i32 = i32::max_value();
        for idx in 0..point_list.len() {
            let p = &point_list[idx];

            let d = p.dist_from(x, y);
            if d < min_dist {
                closest_pts.clear();
                closest_pts.push(idx);

                min_dist = d;
            } else if d == min_dist {
                closest_pts.push(idx);
            }
        }
        if closest_pts.len() == 1 {
            let count = point_areas.entry(closest_pts[0]).or_insert(0);
            *count += 1;
        }

    } }

    // now figure out which ones are on the edges so that they're infinite
    let mut infinite_points: HashSet<_> = HashSet::new();
    let mut edge: Vec<(i32, i32)> = Vec::new();
    edge.extend(
        (0..max_x+1).map(|x| (x, 0))
    );
    edge.extend(
        (0..max_y+1).map(|y| (max_x+1, y))
    );
    edge.extend(
        (1..max_x+2).map(|x| (x, max_y+1))
    );
    edge.extend(
        (1..max_y+2).map(|y| (0, y))
    );
    for edge_pt in edge.iter() {
        let (x, y) = edge_pt;

        let mut closest_pts: Vec<usize> = Vec::new();
        let mut min_dist: i32 = i32::max_value();
        for idx in 0..point_list.len() {
            let p = &point_list[idx];

            let d = p.dist_from(*x, *y);
            if d < min_dist {
                closest_pts.clear();
                closest_pts.push(idx);

                min_dist = d;
            } else if d == min_dist {
                closest_pts.push(idx);
            }
        }
        if closest_pts.len() == 1 {
            infinite_points.insert(closest_pts[0]);
        }
    }

    let noninfinite_pts: HashMap<_, _> = point_areas.iter().filter(
        |(k, _v)| !infinite_points.contains(k)
    ).collect();
    let max_area = noninfinite_pts.values().max().unwrap();

    **max_area
}

pub fn q2(fname: String) -> i32 {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let point_list: Vec<_> = f_str.lines().map(|x: &str| {
        let split_str: Vec<i32> = x.split(", ").map(|n_str: &str| {
            n_str.parse().unwrap()
        }).collect();
        Point::new(split_str[0], split_str[1])
    }).collect();

    let max_x: i32 = point_list.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y: i32 = point_list.iter().max_by_key(|p| p.y).unwrap().y;

    let mut total_area: i32 = 0;
    for x in 0..max_x { for y in 0..max_y {
        let total_dist: i32 = point_list.iter().map(|ref p| {
            p.dist_from(x, y)
        }).sum();

        if total_dist < 10_000 {
            total_area += 1;
        }
    } }

    total_area
}
