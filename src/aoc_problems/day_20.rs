use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::usize;

use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
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

    fn to_the(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate { x: self.x, y: self.y + 1 },
            Direction::Down => Coordinate { x: self.x, y: self.y - 1 },
            Direction::Left => Coordinate { x: self.x - 1, y: self.y },
            Direction::Right => Coordinate { x: self.x + 1, y: self.y },
        }
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

struct RoomPlan {
    room_links: HashMap<Coordinate, Vec<Coordinate>>
}

impl RoomPlan {
    fn new() -> Result<RoomPlan> {
        // (0, 0) is marked as a starting room
        Ok(
            RoomPlan {
                room_links: HashMap::new()
            }
        )
    }

    fn add_link_between_rooms(&mut self, from: Coordinate, to: Coordinate) {
        let rooms_adjacent_to_current = self.room_links.entry(from).or_insert(Vec::new());
        rooms_adjacent_to_current.push(to);
    }

    fn _parse_regex(&mut self, coord: Coordinate, _path_regex: &[char]) -> Result<Coordinate> {
        let mut current_coord = coord;
        let mut current_chars = _path_regex;

        while current_chars.len() != 0 {
            match current_chars[0] {
                'N' | 'E' | 'W' | 'S' => {
                    let direction = match current_chars[0] {
                        'N' => Direction::Up,
                        'E' => Direction::Right,
                        'W' => Direction::Left,
                        'S' => Direction::Down,
                        dir => return err!("Direction cannot be parsed: {}", dir)
                    };

                    let next_coord = current_coord.to_the(direction);

                    self.add_link_between_rooms(current_coord, next_coord);
                    self.add_link_between_rooms(next_coord, current_coord);

                    current_coord = next_coord;
                    current_chars = &current_chars[1..];
                },
                '(' => {
                    let mut bracket_scope_count: usize = 0;
                    let mut idxs_to_split: Vec<usize> = vec![0];
                    let mut end_idx: usize = 0;
                    for (idx, c) in current_chars.iter().enumerate() {
                        match c {
                            '(' => {
                                bracket_scope_count += 1;
                            },
                            ')' => {
                                bracket_scope_count -= 1;
                                if bracket_scope_count == 0 {
                                    end_idx = idx;
                                    idxs_to_split.push(end_idx);
                                    break;
                                }
                            },
                            '|' => {
                                if bracket_scope_count == 1 {
                                    idxs_to_split.push(idx);
                                }
                            },
                            'N' | 'E' | 'W' | 'S' => {},
                            _ => return err!("Cannot determine char in scope")
                        }
                    }

                    let finished_coords: Result<HashSet<Coordinate>> = idxs_to_split[..].windows(2)
                        .map(|idxs| &current_chars[(idxs[0]+1)..idxs[1]])
                        .map(|cs: &[char]| self._parse_regex(current_coord, cs))
                        .collect();


                    #[allow(unused_variables)]
                    let finished_coords = finished_coords?;

                    current_chars = &current_chars[end_idx+1..];

                    // TODO: figure out how to deal with differing paths
                },
                ')' => {
                    return err!("Shouldn't try to parse closing bracket")
                },
                x => {
                    return err!("Cannot parse {}", x)
                }
            }
        }

        Ok(current_coord)
    }

    fn parse_regex(&mut self, path_regex: &[char]) -> Result<()> {
        let starting_coord = Coordinate::new(0, 0);

        self._parse_regex(starting_coord, path_regex)?;

        Ok(())
    }

    fn path_length_to_furthest_room(&self) -> Result<usize> {
        let mut d: HashMap<Coordinate, usize> = HashMap::new();
        d.insert(Coordinate::new(0, 0), 0);

        let mut queue: VecDeque<Coordinate> = VecDeque::new();
        queue.push_front(Coordinate::new(0, 0));
        let mut todo_set: HashSet<Coordinate> = HashSet::new();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        while let Some(c) = queue.pop_front() {
            todo_set.remove(&c);
            visited.insert(c);

            for neighbour in self.room_links.get(&c).ok_or("Cannot find room in map".to_string())? {
                if visited.contains(neighbour) {
                    continue;
                }
                if !todo_set.contains(&neighbour) {
                    queue.push_back(*neighbour);
                    todo_set.insert(*neighbour);
                }

                let new_dist = 1 + *d.get(&c).unwrap_or(&0);
                if !d.contains_key(neighbour) || new_dist < d[neighbour] {
                    d.insert(*neighbour, new_dist);
                }
            }
        }

        let max_distance = d.values().max().ok_or("No rooms in distance map".to_string())?;
        Ok(*max_distance)
    }

    fn number_more_than_n_away(&self, n: usize) -> Result<usize> {
        let mut d: HashMap<Coordinate, usize> = HashMap::new();
        d.insert(Coordinate::new(0, 0), 0);

        let mut queue: VecDeque<Coordinate> = VecDeque::new();
        queue.push_front(Coordinate::new(0, 0));
        let mut todo_set: HashSet<Coordinate> = HashSet::new();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        while let Some(c) = queue.pop_front() {
            todo_set.remove(&c);
            visited.insert(c);

            for neighbour in self.room_links.get(&c).ok_or("Cannot find room in map".to_string())? {
                if visited.contains(neighbour) {
                    continue;
                }
                if !todo_set.contains(&neighbour) {
                    queue.push_back(*neighbour);
                    todo_set.insert(*neighbour);
                }

                let new_dist = 1 + *d.get(&c).unwrap_or(&0);
                if !d.contains_key(neighbour) || new_dist < d[neighbour] {
                    d.insert(*neighbour, new_dist);
                }
            }
        }

        Ok(d.values().filter(|&&dd| dd >= n).count())
    }
}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    f_contents = f_contents.trim().to_string();

    _q1(f_contents).unwrap()
}

fn _q1(path_regex: String) -> Result<usize> {
    let mut room_plan = RoomPlan::new()?;

    if !(path_regex.starts_with("^") && path_regex.ends_with("$")) {
        println!("{}", path_regex);
        return err!("Input regex does not start with ^ and end with $");
    }

    let path_regex: Vec<char> = path_regex.chars().collect();

    room_plan.parse_regex(&path_regex[1..path_regex.len()-1])?;
    room_plan.path_length_to_furthest_room()
}

pub fn q2(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    f_contents = f_contents.trim().to_string();

    _q2(f_contents).unwrap()
}

fn _q2(path_regex: String) -> Result<usize> {
    let mut room_plan = RoomPlan::new()?;

    if !(path_regex.starts_with("^") && path_regex.ends_with("$")) {
        println!("{}", path_regex);
        return err!("Input regex does not start with ^ and end with $");
    }

    let path_regex: Vec<char> = path_regex.chars().collect();

    room_plan.parse_regex(&path_regex[1..path_regex.len()-1])?;
    room_plan.number_more_than_n_away(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_q1_test1() {
        assert_eq!(
            _q1("^WNE$".to_string()).unwrap(), 3
        );
    }

    #[test]
    fn day20_q1_test2() {
        assert_eq!(
            _q1("^ENWWW(NEEE|SSE(EE|N))$".to_string()).unwrap(), 10
        );
    }

    #[test]
    fn day20_q1_test3() {
        assert_eq!(
            _q1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()).unwrap(), 18
        );
    }

    #[test]
    fn day20_q1_test4() {
        assert_eq!(
            _q1(
                "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
                .to_string()
            ).unwrap(), 23
        );
    }

    #[test]
    fn day20_q1_test5() {
        assert_eq!(
            _q1(
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
                .to_string()
            ).unwrap(), 31
        );
    }
}
