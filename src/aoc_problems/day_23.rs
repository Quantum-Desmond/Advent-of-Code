use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet};

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

    fn distance_from(&self, other: &Self) -> usize {
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
        self.pos.distance_from(&other.pos)
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
            println!("Cannot parse: {}", s);
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

}
