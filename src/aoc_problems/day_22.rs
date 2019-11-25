use std::cmp;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::result;
use std::usize;

use std::collections::BTreeMap;

type Result<T> = result::Result<T, Box<dyn Error>>;

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
    x: usize,
    y: usize
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn surrounding_squares(self: Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate { x: self.x - 1, y: self.y - 1 },
            Coordinate { x: self.x, y: self.y - 1 },
            Coordinate { x: self.x + 1, y: self.y - 1 },
            Coordinate { x: self.x - 1, y: self.y },
            Coordinate { x: self.x + 1, y: self.y },
            Coordinate { x: self.x - 1, y: self.y + 1 },
            Coordinate { x: self.x, y: self.y + 1 },
            Coordinate { x: self.x + 1, y: self.y + 1 },
        ]
    }

    fn adjacent_squares(self) -> Vec<Coordinate> {
        vec![
            Coordinate { x: self.x - 1, ..self },
            Coordinate { x: self.x + 1, ..self },
        ]
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<cmp::Ordering> {
        Some((self.y, self.x).cmp(&(other.y, other.x)))
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum CaveType {
    Rocky,
    Narrow,
    Wet
}

impl CaveType {
    fn new(erosion_lvl: usize) -> CaveType {
        match erosion_lvl % 3 {
            0 => CaveType::Rocky,
            1 => CaveType::Wet,
            2 => CaveType::Narrow,
            _ => panic!("Not possible!")
        }
    }

    fn to_risk_lvl(&self) -> usize {
        match &self {
            CaveType::Rocky => 0,
            CaveType::Wet => 1,
            CaveType::Narrow => 2,
        }
    }
}

impl fmt::Display for CaveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CaveType::Rocky => write!(f, "."),
            CaveType::Narrow => write!(f, "="),
            CaveType::Wet => write!(f, "|"),
        }
    }
}

struct Caves {
    depth: usize,
    target: Coordinate,
    geologic_idx_map: BTreeMap<Coordinate, usize>,
    caves_types: BTreeMap<Coordinate, CaveType>
}

impl Caves {
    fn new(depth: usize, target: Coordinate) -> Caves {
        Caves {
            depth,
            target,
            geologic_idx_map: BTreeMap::new(),
            caves_types: BTreeMap::new()
        }
    }

    fn calculate_erosion_lvl(&mut self, c: Coordinate) -> usize {
        if let Some(&n) = self.geologic_idx_map.get(&c) {
            return n;
        }

        let geologic_idx = match (c.x, c.y) {
            (0, 0) => 0,
            (x, y) if x == self.target.x && y == self.target.y => 0,
            (x, 0) => 16807 * x,
            (0, y) => 48271 * y,
            (x, y) => self.calculate_erosion_lvl(Coordinate::new(x-1, y)) * self.calculate_erosion_lvl(Coordinate::new(x, y-1))
        };

        // Take modulo as the extra not needed
        // let geologic_idx = geologic_idx % 20183;
        let erosion_lvl = (geologic_idx + self.depth) % 20183;

        self.geologic_idx_map.insert(c, erosion_lvl);
        erosion_lvl
    }

    fn calculate_erosion_levels(&mut self) {
        for y in 0..self.target.y+1 {
            for x in 0..self.target.x+1 {
                let coord = Coordinate::new(x, y);
                let erosion_level = self.calculate_erosion_lvl(coord);
                self.caves_types.insert(coord, CaveType::new(erosion_level));
            }
        }
    }

    fn total_risk_level(&self) -> usize {
        self.caves_types.values().map(|cave_type| cave_type.to_risk_lvl()).sum()
    }
}

impl fmt::Display for Caves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (c, acre) in &self.caves_types {
            write!(f, "{}", acre)?;
            if c.x == self.target.x {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

pub fn q1(depth: usize, target_x: usize, target_y: usize) -> usize {
    let mut caves = Caves::new(depth, Coordinate{x: target_x, y: target_y});

    caves.calculate_erosion_levels();

    caves.total_risk_level()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_q1_test() {
        assert_eq!(
            q1(510, 10, 10),
            114
        );
    }
}
