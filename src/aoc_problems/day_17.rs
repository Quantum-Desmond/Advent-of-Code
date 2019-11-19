use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;
use std::usize;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use regex::Regex;

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

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Species::GOBLIN => write!(f, "Goblin"),
            Species::ELF => write!(f, "Elf"),
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn surrounding_squares(self: Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate { y: self.y - 1, ..self },
            Coordinate { x: self.x - 1, ..self },
            Coordinate { x: self.x + 1, ..self },
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
    fn new(species: Species, atk: usize) -> Character {
        Character {
            species,
            health: 200, attack: atk}
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
    max: Coordinate,
    elf_atk: usize,
    initial_elf_count: usize
}

impl Map {
    fn new(input_grid: Vec<Vec<char>>) -> Result<Map> {
        Map::new_w_atk(input_grid, 3)
    }

    fn new_w_atk(input_grid: Vec<Vec<char>>, elf_atk: usize) -> Result<Map> {
        let mut map = Map::default();
        map.max = Coordinate { x: input_grid[0].len(), y: input_grid.len() };

        let goblin_atk = 3;

        for y in 0..input_grid.len() {
            for x in 0..input_grid[y].len() {
                let c = Coordinate { x, y };
                match input_grid[y][x] {
                    'G' => {
                        map.characters.insert(c, Character::new(Species::GOBLIN, goblin_atk));
                        map.grid.insert(c, Cell::Open);
                    },
                    'E' => {
                        map.characters.insert(c, Character::new(Species::ELF, elf_atk));
                        map.grid.insert(c, Cell::Open);
                    },
                    cell => {
                        map.grid.insert(c, cell.to_string().parse()?);
                    }
                }
            }
        }

        map.initial_elf_count = map.characters.values().filter(|c| c.species == Species::ELF).count();
        map.elf_atk = elf_atk;
        println!("Elf attack = {}", elf_atk);

        Ok(map)
    }

    fn outcome(&mut self) -> Result<usize> {
        const LIMIT: usize = 1000;
        for i in 0..LIMIT {
            let run_again = self.increment();
            if !run_again {
                println!("Number of loops = {}", i);
                return Ok(i * self.total_health());
            }

        }

        return err!("Limit surpassed!");
    }

    fn any_elves_lost(&self) -> bool {
        self.initial_elf_count != self.characters.values().filter(|c| c.species == Species::ELF).count()
    }

    fn elfy_outcome(&mut self) -> Option<usize> {
        let outcome = self.outcome();

        match self.any_elves_lost() {
            true => None,
            false => outcome.ok()
        }
    }

    fn possible_targets(&self, c: Coordinate) -> Vec<Coordinate> {
        let current_species: Species = self.characters.get(&c).unwrap().species;
        self.characters
            .keys()
            .cloned()
            .filter(|coord| self.characters.get(&coord).unwrap().species != current_species)
            .collect()
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
        queue.push_front(start);
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
        let mut possible_targets: Vec<_> = coord
            .surrounding_squares()
            .into_iter()
            .filter(|c| {
                self.grid.get(&c).unwrap().is_open()
                    && self.characters.contains_key(&c)
                    && self.characters.get(&c).unwrap().species != self.characters.get(&coord).unwrap().species
            })
            .collect();

        possible_targets.sort();

        possible_targets.into_iter().min_by_key(|&c| self.characters.get(&c).unwrap().health)
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

                let mut ordered_dists: Vec<_> = adjacent_squares
                    .iter()
                    .filter_map(|target| reachable_distances.get(&target).map(|d| (target, d)))
                    .collect();
                ordered_dists.sort_by_key(|&(c, _)| c);

                let chosen_coord = ordered_dists
                    .into_iter()
                    .min_by_key(|&(_, d)| d)
                    .map(|(c, _)| c);

                let chosen_coord = match chosen_coord {
                    None => {
                        continue
                    },
                    Some(c) => c
                };

                let dists_from_target = self.distances_from(*chosen_coord);
                let mut ordered_dists_to_move_to: Vec<_> = self.free_squares_around(current_coord)
                    .into_iter()
                    .filter_map(|target| dists_from_target.get(&target).map(|d| (target, d)))
                    .collect();
                ordered_dists_to_move_to.sort_by_key(|&(c, _)| c);

                let coord_to_move_to = ordered_dists_to_move_to
                    .into_iter()
                    .min_by_key(|&(_, d)| d)
                    .map(|(c, _)| c)
                    .unwrap();

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

        println!("Total health for everyone: {:?}", self.characters.iter().map(|(k, v)| (k, v.health)).collect::<Vec<_>>());

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

enum WaterType {
    Still,
    Flowing
}

impl fmt::Display for WaterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaterType::Still => write!(f, "~"),
            WaterType::Flowing => write!(f, "|"),
        }
    }
}

enum Material {
    Clay,
    Sand,
    Spring,
    Water(WaterType)
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Material::Clay => write!(f, "#"),
            Material::Sand => write!(f, "."),
            Material::Spring => write!(f, "+"),
            Material::Water(water_type) => write!(f, "{}", water_type),
        }
    }
}

struct Underground {
    min_coord: Coordinate,
    max_coord: Coordinate,
    material_grid: BTreeMap<Coordinate, Material>
}

impl Underground {
    fn new(sand_locations: Vec<String>) -> Underground {
        let sand_regex = Regex::new(r"^(\w)=(\d+), (\w)=(\d+)..(\d+)$").unwrap();
        let mut material_grid: BTreeMap<Coordinate, Material> = BTreeMap::new();
        for sand_location in sand_locations {
            let cap = sand_regex.captures(&sand_location).unwrap();
            match &cap[1] {
                "x" => {
                    if &cap[3] != "y" {
                        panic!("Second letter not y");
                    }

                    let x_coord: usize = cap[2].parse().unwrap();
                    let first_y: usize = cap[4].parse().unwrap();
                    let last_y: usize = cap[5].parse().unwrap();

                    for y_coord in first_y..last_y+1 {
                        material_grid.insert(Coordinate{ x: x_coord, y: y_coord }, Material::Clay);
                    }
                },
                "y" => {
                    if &cap[3] != "x" {
                        panic!("Second letter not x");
                    }

                    let y_coord: usize = cap[2].parse().unwrap();
                    let first_x: usize = cap[4].parse().unwrap();
                    let last_x: usize = cap[5].parse().unwrap();

                    for x_coord in first_x..last_x+1 {
                        material_grid.insert(Coordinate{ x: x_coord, y: y_coord }, Material::Clay);
                    }
                },
                _ => panic!("Cannot read coordinate!")
            }
        }

        let mut min_coord: Coordinate = Coordinate { x: usize::MAX, y: usize::MAX };
        let mut max_coord: Coordinate = Coordinate { x: 0, y: 0 };
        match material_grid.keys().map(|c| c.x).minmax() {
            MinMax(min_x, max_x) => {
                min_coord.x = min_x;
                max_coord.x = max_x;
            },
            _ => {
                panic!("No distinct min and max for x");
            }
        }

        match material_grid.keys().map(|c| c.y).minmax() {
            MinMax(min_y, max_y) => {
                min_coord.y = min_y;
                max_coord.y = max_y;
            },
            _ => {
                panic!("No distinct min and max for x");
            }
        }

        // Account for flowing of water
        min_coord.x -= 2;
        max_coord.x += 2;

        // Fill the rest of the grid in with sand
        for y in 0..max_coord.y+1 {
            for x in min_coord.x..max_coord.x+1 {
                let c: Coordinate = Coordinate { x, y };
                if !material_grid.contains_key(&c) {
                    material_grid.insert(c, Material::Sand);
                }
            }
        }

        material_grid.insert(Coordinate { x: 500, y: 0 }, Material::Spring);

        println!("Ranges: from {} to {}", min_coord, max_coord);

        Underground {
            material_grid,
            min_coord,
            max_coord
        }
    }
}

impl fmt::Display for Underground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (c, cell) in &self.material_grid {
            write!(f, "{}", cell)?;
            if c.x == self.max_coord.x {
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
    let sand_locations: Vec<String> = f_contents.lines().map(|x: &str| {
        x.trim().to_string()
    }).collect();

    _q1(sand_locations).unwrap()
}

fn _q1(sand_locations: Vec<String>) -> Result<usize> {
    let underground = Underground::new(sand_locations);

    print!("{}", underground);

    // unimplemented!();
    Ok(0)
}

pub fn q2(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let map: Vec<Vec<char>> = f_contents.lines().map(|x: &str| {
        x.to_string().chars().collect::<Vec<char>>()
    }).collect();

    _q2(map).unwrap()
}

fn _q2(input_grid: Vec<Vec<char>>) -> Result<usize> {
    for elf_atk in 4..100 {
        match Map::new_w_atk(input_grid.clone(), elf_atk).unwrap().elfy_outcome() {
            Some(outcome) => {
               return Ok(outcome)
            },
            None => continue
        }
    }

    return err!("Limit surpassed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_test() {
        assert_eq!(
            _q1(vec![
                "x=495, y=2..7".to_string(),
                "y=7, x=495..501".to_string(),
                "x=501, y=3..7".to_string(),
                "x=498, y=2..4".to_string(),
                "x=506, y=1..2".to_string(),
                "x=498, y=10..13".to_string(),
                "x=504, y=10..13".to_string(),
                "y=13, x=498..504".to_string(),
            ]).unwrap(),
            57
        );
    }
}
