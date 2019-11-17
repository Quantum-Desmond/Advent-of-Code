use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Species {
    GOBLIN,
    ELF
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn surrounding_squares(self: Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate { x: self.x - 1, ..self },
            Coordinate { x: self.x + 1, ..self },
            Coordinate { y: self.y - 1, ..self },
            Coordinate { y: self.y + 1, ..self },
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Character {
    species: Species,
    health: usize,
    attack: usize
}

impl Character {
    fn new(species: Species) -> Character {
        Character {
            species,
            health: 200, attack: 3}
    }

    fn take_damage(&mut self, atk: usize) -> bool {
        if atk >= self.health {
            return true;
        }

        self.health -= atk;

        return false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Open,
    Wall
}

impl Cell {
    fn is_open(self) -> bool {
        self == Cell::Open
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Open => write!(f, "."),
            Cell::Wall => write!(f, "#"),
        }
    }
}

impl FromStr for Cell {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Cell> {
        match s.as_bytes().get(0) {
            None => err!("empty string doesn't work"),
            Some(&b'.') => Ok(Cell::Open),
            Some(&b'#') => Ok(Cell::Wall),
            Some(&b) => err!("Cannot read: 0x{:X}", b),
        }
    }
}

#[derive(Default)]
struct Map {
    grid: BTreeMap<Coordinate, Cell>,
    characters: BTreeMap<Coordinate, Character>,
    max: Coordinate
}

impl Map {
    fn new(input_grid: Vec<Vec<char>>) -> Result<Map> {
        let mut map = Map::default();
        map.max = Coordinate { x: input_grid[0].len(), y: input_grid.len() };

        for y in 0..input_grid.len() {
            for x in 0..input_grid[y].len() {
                let c = Coordinate { x, y };
                match input_grid[y][x] {
                    'G' => {
                        map.characters.insert(c, Character::new(Species::GOBLIN));
                        map.grid.insert(c, Cell::Open);
                    },
                    'E' => {
                        map.characters.insert(c, Character::new(Species::ELF));
                        map.grid.insert(c, Cell::Open);
                    },
                    cell => {
                        map.grid.insert(c, cell.to_string().parse()?);
                    }
                }
            }
        }
        Ok(map)
    }

    fn possible_targets(&self, c: Coordinate) -> Vec<Coordinate> {
        let current_species: Species = self.characters.get(&c).unwrap().species;
        self.characters.keys().cloned().filter(|coord| self.characters.get(&coord).unwrap().species != current_species).collect()
    }

    fn free_squares_around(&self, target: Coordinate) -> Vec<Coordinate> {
        target
            .surrounding_squares()
            .into_iter()
            .filter(|c| self.grid.get(&c).unwrap().is_open() && !self.characters.contains_key(&c))
            .collect()
    }

    fn adjacent_squares_to(&self, targets: &Vec<Coordinate>) -> Vec<Coordinate> {
        targets.iter().cloned().map(|target| self.free_squares_around(target)).flatten().collect()
    }

    fn is_adjacent_to_target(&self, coord: Coordinate, targets: &Vec<Coordinate>) -> bool {
        targets.iter().map(|c| c.surrounding_squares()).flatten().find(|&c| c == coord).is_some()
    }

    fn distances_from(&self, start: Coordinate) -> BTreeMap<Coordinate, usize> {
        let mut d = BTreeMap::new();
        d.insert(start, 0);

        let mut queue: VecDeque<Coordinate> = VecDeque::new();
        let mut todo_set: BTreeSet<Coordinate> = BTreeSet::new();
        let mut visited: BTreeSet<Coordinate> = BTreeSet::new();
        while let Some(c) = queue.pop_front() {
            todo_set.remove(&c);
            visited.insert(c);

            for neighbour in self.free_squares_around(c) {
                if visited.contains(&neighbour) {
                    continue;
                }
                if !todo_set.contains(&neighbour) {
                    queue.push_back(neighbour);
                    todo_set.insert(neighbour);
                }

                let new_dist = 1 + *d.get(&c).unwrap_or(&0);
                if !d.contains_key(&neighbour) || new_dist < d[&neighbour] {
                    d.insert(neighbour, new_dist);
                }
            }
        }
        d
    }

    fn next_target(&self, coord: Coordinate) -> Option<Coordinate> {
        coord.surrounding_squares().into_iter().filter(|c| {
            self.grid.get(&c).unwrap().is_open()
                && self.characters.contains_key(&c)
                && self.characters.get(&c).unwrap().species != self.characters.get(&coord).unwrap().species
        }).min_by_key(|&c| self.characters.get(&c).unwrap().health)
    }

    fn attack(&mut self, attacker: Coordinate, target: Coordinate) {
        let attack_power = self.characters.get(&attacker).unwrap().attack;

        let succumb = self.characters.get_mut(&target).unwrap().take_damage(attack_power);
        if succumb {
            self.characters.remove(&target);
        }
    }

    fn increment(&mut self) -> bool {
        let character_coords: Vec<_> = self.characters.keys().cloned().collect();
        let mut something_happened: bool = false;
        for coord in character_coords.into_iter() {
            let mut current_coord = coord;
            if !self.characters.contains_key(&current_coord) {
                continue;
            }

            let targets = self.possible_targets(current_coord);
            if targets.len() == 0 {
                return false;
            }

            let adjacent_squares = self.adjacent_squares_to(&targets);
            if adjacent_squares.len() == 0 && !self.is_adjacent_to_target(current_coord, &targets) {
                continue;
            }

            if !self.is_adjacent_to_target(current_coord, &targets) {
                // it moves
                let reachable_distances = self.distances_from(current_coord);
                let chosen_coord = targets
                    .iter()
                    .filter_map(|target| reachable_distances.get(&target).map(|d| (target, d)))
                    .min_by_key(|&(_, d)| d)
                    .map(|(c, _)| c);

                let chosen_coord = match chosen_coord {
                    None => continue,
                    Some(c) => c
                };

                let dists_from_target = self.distances_from(*chosen_coord);
                let coord_to_move_to = self.free_squares_around(current_coord)
                    .into_iter()
                    .filter_map(|target| dists_from_target.get(&target).map(|d| (target, d)))
                    .min_by_key(|&(_, d)| d)
                    .map(|(c, _)| c)
                    .unwrap();

                if !self.characters.contains_key(&coord_to_move_to) {
                    println!("baaaaaad");
                    panic!();
                }
                let character = self.characters.remove(&current_coord).unwrap();
                self.characters.insert(coord_to_move_to, character);

                current_coord = coord_to_move_to;

                something_happened = true;
            }

            // attack
            let target_to_attack = match self.next_target(current_coord) {
                None => continue,
                Some(c) => c
            };

            self.attack(current_coord, target_to_attack);

            something_happened = true;
        }
        something_happened
    }

    fn total_health(&self) -> usize {
        // first, check only one species is left
        let species_left: Vec<Species> = self.characters.values().map(|character| character.species).collect();
        if !species_left.iter().all(|&s| species_left[0] == s) {
            write!(io::stdout(), "{}", self).unwrap();
            panic!("More than one species left");
        }

        self.characters.values().map(|character| character.health).sum()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (c, cell) in &self.grid {
            if let Some(character) = self.characters.get(&c) {
                match character.species {
                    Species::GOBLIN => {
                        write!(f, "G")?;
                    },
                    Species::ELF => {
                        write!(f, "E")?;
                    }
                }
            } else {
                write!(f, "{}", cell)?;
            }
            if c.x == self.max.x - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let map: Vec<Vec<char>> = f_contents.lines().map(|x: &str| {
        x.to_string().chars().collect::<Vec<char>>()
    }).collect();

    _q1(map).unwrap()
}

fn _q1(input_grid: Vec<Vec<char>>) -> Result<usize> {
    let mut map = Map::new(input_grid).unwrap();

    for (c, character) in &map.characters {
        println!(
            "{} at {}",
            match character.species {
                Species::ELF => "Elf",
                Species::GOBLIN => "Goblin",
            },
            c
        );
    }

    const LIMIT: usize = 1000;
    for i in 0..LIMIT {
        let run_again = map.increment();
        if !run_again {
            println!("Number of loops = {}", i);
            return Ok(i * map.total_health());
        }
    }

    return err!("Limit surpassed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_test1() {
        assert_eq!(
            _q1(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'G', '.', '.', '#', 'E', '#'],
                vec!['#', 'E', '#', 'E', '.', 'E', '#'],
                vec!['#', 'G', '.', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', '#', 'E', '#'],
                vec!['#', '.', '.', '.', 'E', '.', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#'],
            ]).unwrap(),
            36334
        );
    }
    #[test]
    fn q1_test2() {
        assert_eq!(
            _q1(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'E', '.', '.', 'E', 'G', '#'],
                vec!['#', '.', '#', 'G', '.', 'E', '#'],
                vec!['#', 'E', '.', '#', '#', 'E', '#'],
                vec!['#', 'G', '.', '.', '#', '.', '#'],
                vec!['#', '.', '.', 'E', '#', '.', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#'],
            ]).unwrap(),
            39514
        );
    }
    #[test]
    fn q1_test3() {
        assert_eq!(
            _q1(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'E', '.', 'G', '#', '.', '#'],
                vec!['#', '.', '#', 'G', '.', '.', '#'],
                vec!['#', 'G', '.', '#', '.', 'G', '#'],
                vec!['#', 'G', '.', '.', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'E', '.', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#'],
            ]).unwrap(),
            27755
        );
    }
    #[test]
    fn q1_test4() {
        assert_eq!(
            _q1(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', '.', 'E', '.', '.', '.', '#'],
                vec!['#', '.', '#', '.', '.', 'G', '#'],
                vec!['#', '.', '#', '#', '#', '.', '#'],
                vec!['#', 'E', '#', 'G', '#', 'G', '#'],
                vec!['#', '.', '.', '.', '#', 'G', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#'],
            ]).unwrap(),
            28944
        );
    }
    #[test]
    fn q1_test5() {
        assert_eq!(
            _q1(vec![
                vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'G', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', 'E', '.', '#', '.', '.', '.', '#'],
                vec!['#', '.', '.', '#', '#', '.', '.', 'G', '#'],
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#'],
                vec!['#', '.', 'G', '.', '.', '.', 'G', '.', '#'],
                vec!['#', '.', '.', '.', '.', '.', 'G', '.', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ]).unwrap(),
            18740
        );
    }
}
