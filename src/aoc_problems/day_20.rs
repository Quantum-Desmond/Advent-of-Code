use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::usize;

use std::collections::BTreeMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

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
    fn new(x: i32, y: i32) -> Coordinate {
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
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

struct RoomPlan {
    room_links: BTreeMap<Coordinate, Vec<Coordinate>>,
    path_regex: Vec<char>
}

impl RoomPlan {
    fn new(path_regex: String) -> Result<RoomPlan> {
        // (0, 0) is marked as a starting room
        if !(path_regex.starts_with("^") && path_regex.ends_with("$")) {
            return err!("Input regex does not start with ^ and end with $");
        }

        Ok(
            RoomPlan {
                room_links: BTreeMap::new(),
                path_regex: path_regex.chars().collect()
            }
        )
    }

    fn _parse_regex(&mut self, coord: Coordinate, start: usize, end: usize) -> Result<Coordinate> {
        let current_coord = coord;

        let current_char_scope = &self.path_regex[start..end];

        Ok(current_coord)
    }

    fn parse_regex(&mut self) -> Result<()> {
        let (start, end) = (1, self.path_regex.len()-1);

        let starting_coord = Coordinate::new(0, 0);

        self._parse_regex(starting_coord, start, end)?;

        Ok(())
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
    room_plan.parse_regex()?;
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
