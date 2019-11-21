use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
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
    x: i32,
    y: i32
}

impl Coordinate {
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
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl Acre {
    fn new(acre_symbol: char) -> Acre {
        match acre_symbol {
            '.' => Acre::Open,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => panic!()
        }
    }

    fn is_open(&self) -> bool {
        match self {
            Acre::Open => true,
            _ => false
        }
    }

    fn is_trees(&self) -> bool {
        match self {
            Acre::Trees => true,
            _ => false
        }
    }
    fn is_lumberyard(&self) -> bool {
        match self {
            Acre::Lumberyard => true,
            _ => false
        }
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Acre::Open => write!(f, "."),
            Acre::Trees => write!(f, "|"),
            Acre::Lumberyard => write!(f, "#"),
        }
    }
}

fn new_acre_at(current_acre: Acre, surrounding_acres: Vec<Acre>) -> Acre {
    match current_acre {
        Acre::Open => {
            // Becomes trees if surrounded by at least 3 acres of trees
            match surrounding_acres.iter().filter(|acre| acre.is_trees()).count() {
                3..=8 => Acre::Trees,
                _ => Acre::Open
            }
        },
        Acre::Trees => {
            // Becomes trees if surrounded by at least 3 acres of trees
            match surrounding_acres.iter().filter(|acre| acre.is_lumberyard()).count() {
                3..=8 => Acre::Lumberyard,
                _ => Acre::Trees
            }
        },
        Acre::Lumberyard => {
            match (
                surrounding_acres.iter().filter(|acre| acre.is_lumberyard()).count(),
                surrounding_acres.iter().filter(|acre| acre.is_trees()).count()
            ) {
                (x, y) if x >= 1 && y >= 1 => Acre::Lumberyard,
                _ => Acre::Open
            }
        },
    }
}

struct Grove {
    acre_grid: BTreeMap<Coordinate, Acre>,
    max_coord: Coordinate
}

impl Grove {
    fn new(grove_rows: Vec<String>) -> Grove {
        let max_coord = Coordinate { x: grove_rows[0].len() as i32 - 1, y: grove_rows.len() as i32 - 1 };
        let mut acre_grid: BTreeMap<Coordinate, Acre> = BTreeMap::new();

        for y in 0..max_coord.y+1 {
            let grove_row: Vec<char> = grove_rows[y as usize].chars().collect();
            for x in 0..max_coord.x+1 {
                acre_grid.insert(Coordinate{x, y}, Acre::new(grove_row[x as usize]));
            }
        }

        Grove {
            acre_grid,
            max_coord
        }
    }

    fn increment_minute(&mut self) {
        let mut new_acre: BTreeMap<Coordinate, Acre> = BTreeMap::new();

        for (&c, &acre) in &self.acre_grid {
            new_acre.insert(
                c,
                new_acre_at(
                    acre,
                    c.surrounding_squares()
                        .into_iter()
                        .filter_map(|square| self.acre_grid.get(&square))
                        .map(|acre| acre.clone())
                        .collect()
                )
            );
        }

        self.acre_grid = new_acre;
    }

    fn resource_value(&self) -> usize {
        self.acre_grid.values().filter(|acre| acre.is_trees()).count()
            * self.acre_grid.values().filter(|acre| acre.is_lumberyard()).count()
    }
}

impl fmt::Display for Grove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (c, acre) in &self.acre_grid {
            write!(f, "{}", acre)?;
            if c.x == self.max_coord.x {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

struct RoomPlan {
    room_links: BTreeMap<Coordinate, Vec<Coordinate>>
}

impl RoomPlan {
    fn new(path_regex: String) -> Result<RoomPlan> {
        // (0, 0) is marked as a starting room
        unimplemented!();
    }

    fn path_length_to_furthest_room(&self) -> usize {
        unimplemented!();
    }
}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");

    _q1(f_contents).unwrap()
}

fn _q1(path_regex: String) -> Result<usize> {
    let mut room_plan = RoomPlan::new(path_regex)?;
    Ok(room_plan.path_length_to_furthest_room())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_test1() {
        assert_eq!(
            _q1("^WNE$".to_string()).unwrap(), 3
        );
    }

    #[test]
    fn q1_test2() {
        assert_eq!(
            _q1("^ENWWW(NEEE|SSE(EE|N))$".to_string()).unwrap(), 10
        );
    }

    #[test]
    fn q1_test3() {
        assert_eq!(
            _q1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()).unwrap(), 18
        );
    }

    #[test]
    fn q1_test4() {
        assert_eq!(
            _q1(
                "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
                .to_string()
            ).unwrap(), 23
        );
    }

    #[test]
    fn q1_test5() {
        assert_eq!(
            _q1(
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
                .to_string()
            ).unwrap(), 31
        );
    }
}
