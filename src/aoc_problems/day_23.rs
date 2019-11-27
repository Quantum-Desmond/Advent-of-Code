use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::result;
use std::ops::{Add, Sub, AddAssign};
use std::str::FromStr;

use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet};

use itertools::Itertools;

use regex::Regex;

type Result<T> = result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Coordinate {
        Coordinate { x, y, z }
    }

    fn surrounding_squares(self) -> Vec<Coordinate> {
        let ds_list: Vec<_> = iter::repeat(-1..2).take(3).multi_cartesian_product().collect();
        ds_list.into_iter()
            // .filter(|ds| ds[0] != 0 || ds[1] != 0 || ds[2] != 0)
            .map(|ds| {
                let dx = ds[0];
                let dy = ds[1];
                let dz = ds[2];
                Coordinate { x: self.x + dx, y: self.y + dy, z: self.z + dz }
            }).collect()
    }

    fn distance_from(&self, other: Self) -> usize {
        (
            (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
        ) as usize
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some((self.z, self.y, self.x).cmp(&(other.z, other.y, other.x)))
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Nanobot {
    pos: Coordinate,
    radius: usize
}

impl Nanobot {
    fn new(pos: Coordinate, radius: usize) -> Nanobot {
        Nanobot { pos, radius }
    }

    fn distance_from(&self, other: &Self) -> usize {
        self.pos.distance_from(other.pos)
    }
    fn is_in_range_of(&self, pos: Coordinate) -> bool {
        self.pos.distance_from(pos) <= self.radius
    }
}

impl FromStr for Nanobot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref NANOBOT_RE: Regex = Regex::new(
                r"pos=<(?P<x>-?[0-9]+),(?P<y>-?[0-9]+),(?P<z>-?[0-9]+)>, r=(?P<r>[0-9]+)"
            ).unwrap();
        }

        if !s.is_ascii() {
            return err!("area must be in ASCII");
        }

        if s.lines().count() != 1 {
            return err!("Only accepts 1 line");
        }

        if let Some(caps) = NANOBOT_RE.captures(s) {
            return Ok(Nanobot::new(
                Coordinate::new(
                    caps["x"].parse()?,
                    caps["y"].parse()?,
                    caps["z"].parse()?
                ),
                caps["r"].parse()?
            ));
        }

        err!("Cannot parse nanobot line: {}", s)
    }
}

impl fmt::Display for Nanobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nanobot at {} with radius {}", self.pos, self.radius)
    }
}

fn nanobots_in_range_of(nanobots: &Vec<Nanobot>, c: Coordinate) -> usize {
    nanobots.iter().filter(|nanobot| nanobot.pos.distance_from(c) <= nanobot.radius).count()
}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    f_contents = f_contents.trim().to_string();

    _q1(f_contents).unwrap()
}

fn _q1(nanobot_list: String) -> Result<usize> {
    let nanobots: Result<Vec<Nanobot>> = nanobot_list
        .trim()
        .lines()
        .map(|l| l.trim().parse())
        .collect();

    let nanobots = nanobots?;

    let best_nanobot = nanobots.iter().max_by_key(|nanobot| nanobot.radius).ok_or("No nanobots!")?;

    Ok(
        nanobots
            .iter()
            .filter(|nanobot| best_nanobot.distance_from(nanobot) <= best_nanobot.radius)
            .count()
    )
}

pub fn q2(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    f_contents = f_contents.trim().to_string();

    _q2(f_contents).unwrap()
}

fn _q2(nanobot_list: String) -> Result<usize> {
    let nanobots: Result<Vec<Nanobot>> = nanobot_list
        .trim()
        .lines()
        .map(|l| l.trim().parse())
        .collect();

    let nanobots = nanobots?;

    let best_nanobot = nanobots.iter().max_by_key(|nanobot| nanobot.radius).ok_or("No nanobots!")?;

    let mut current_position: Coordinate = best_nanobot.pos;
    let mut nanobots_in_range = nanobots_in_range_of(&nanobots, current_position);
    loop {
        let mut distance_to_move = Coordinate::new(0, 0, 0);
        for nanobot in &nanobots {
            if !nanobot.is_in_range_of(current_position) {
                let relative_coordinate = nanobot.pos - current_position;
                let max_coord_dist = vec![
                    relative_coordinate.x,
                    relative_coordinate.y,
                    relative_coordinate.z,
                ].into_iter().map(|n| n.abs()).max().ok_or("No elements!")?;

                if relative_coordinate.x.abs() == max_coord_dist {
                    distance_to_move += Coordinate::new(relative_coordinate.x.signum(), 0, 0);
                } else if relative_coordinate.y.abs() == max_coord_dist {
                    distance_to_move += Coordinate::new(0, relative_coordinate.y.signum(), 0);
                } else if relative_coordinate.z.abs() == max_coord_dist {
                    distance_to_move += Coordinate::new(0, 0, relative_coordinate.z.signum());
                }
            }
        }

        // let distance_to_move = Coordinate {
        //     x: distance_to_move.x.signum(),
        //     y: distance_to_move.y.signum(),
        //     z: distance_to_move.z.signum(),
        // };

        if distance_to_move == Coordinate::new(0, 0, 0) {
            break;
        }

        current_position += distance_to_move;

        if nanobots_in_range_of(&nanobots, current_position) < nanobots_in_range {
            break;
        }

        nanobots_in_range = nanobots_in_range_of(&nanobots, current_position);
    }

    println!("Got to first part");
    println!("Current position = {}", current_position);

    loop {
        let max_nanobots_in_range: usize = current_position
            .surrounding_squares()
            .into_iter()
            .map(|coord| nanobots_in_range_of(&nanobots, coord))
            .max()
            .ok_or("No coordinates surrounding position")?;

        let mut coords_to_go_to: Vec<Coordinate> = current_position
            .surrounding_squares()
            .into_iter()
            .filter(|&c| nanobots_in_range_of(&nanobots, c) == max_nanobots_in_range)
            .collect();
        coords_to_go_to.sort_by_key(|c| c.distance_from(Coordinate::new(0, 0, 0)));

        let coord_to_go_to: Coordinate = *coords_to_go_to.get(0).ok_or("No coordinates!")?;
        if coord_to_go_to == current_position {
            break;
        }

        current_position = coord_to_go_to;

        println!("Current position is now {} with nanobots {}", current_position, max_nanobots_in_range);
    }

    println!("Position chosen = {}", current_position);
    println!("{}", nanobots_in_range_of(&nanobots, Coordinate::new(11, 12, 12)));
    println!("{}", nanobots_in_range_of(&nanobots, Coordinate::new(12, 12, 12)));

    Ok(
        current_position.distance_from(Coordinate::new(0, 0, 0))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_q1() {
        assert_eq!(
            _q1("
                pos=<0,0,0>, r=4
                pos=<1,0,0>, r=1
                pos=<4,0,0>, r=3
                pos=<0,2,0>, r=1
                pos=<0,5,0>, r=3
                pos=<0,0,3>, r=1
                pos=<1,1,1>, r=1
                pos=<1,1,2>, r=1
                pos=<1,3,1>, r=1
            ".to_string()).unwrap(), 7
        );
    }

    #[test]
    fn day23_q2() {
        assert_eq!(
            _q2("
                pos=<10,12,12>, r=2
                pos=<12,14,12>, r=2
                pos=<16,12,12>, r=4
                pos=<14,14,14>, r=6
                pos=<50,50,50>, r=200
                pos=<10,10,10>, r=5
            ".to_string()).unwrap(), 36
        );
    }
}
