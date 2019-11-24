use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::usize;

use std::collections::BTreeMap;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use regex::Regex;

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
    fn surrounding_squares(self: Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate { y: self.y - 1, ..self },
            Coordinate { x: self.x - 1, ..self },
            Coordinate { x: self.x + 1, ..self },
            Coordinate { y: self.y + 1, ..self },
        ]
    }

    fn square_below(self) -> Coordinate {
        Coordinate { y: self.y + 1, ..self }
    }

    fn adjacent_squares(self) -> Vec<Coordinate> {
        vec![
            Coordinate { x: self.x - 1, ..self },
            Coordinate { x: self.x + 1, ..self },
        ]
    }

    fn square_to_the(self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Left => Coordinate { x: self.x - 1, ..self },
            Direction::Right => Coordinate { x: self.x + 1, ..self },
        }
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
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Material {
    Clay,
    Sand,
    Spring,
    Water(WaterType)
}

impl Material {
    fn is_water(&self) -> bool {
        match &self {
            Material::Water(_) => true,
            _ => false
        }
    }

    fn is_still_water(&self) -> bool {
        match &self {
            Material::Water(WaterType::Still) => true,
            _ => false
        }
    }

    fn is_flowing_water(&self) -> bool {
        match &self {
            Material::Water(WaterType::Flowing) => true,
            _ => false
        }
    }

    fn is_sand(&self) -> bool {
        match &self {
            Material::Sand => true,
            _ => false
        }
    }

    fn can_stay_on(&self) -> bool {
        match &self {
            Material::Clay | Material::Water(WaterType::Still) => true,
            _ => false
        }
    }
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
        min_coord.x -= 1;
        max_coord.x += 1;

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

    fn overflow_spring(&mut self) {
        self.material_grid.insert(Coordinate { x: 500, y: 1 }, Material::Water(WaterType::Flowing));
    }

    fn make_water_flow(&mut self) -> bool {
        let mut change_happened: bool = false;

        let flowing_water_coordinates: Vec<_> = self.material_grid.iter().filter(|(_, material)| material.is_flowing_water()).map(|(&c, _)| c).collect();
        for flowing_water_coord in flowing_water_coordinates {
            let coord_below = flowing_water_coord.square_below();

            if coord_below > self.max_coord {
                continue;
            }

            let material_below_flow = self.material_grid.get(&coord_below);

            match material_below_flow {
                Some(Material::Water(WaterType::Flowing)) => {
                    continue
                },
                Some(Material::Sand) => {
                    self.material_grid.insert(coord_below, Material::Water(WaterType::Flowing));
                    change_happened = true;
                },
                Some(Material::Clay) | Some(Material::Water(WaterType::Still)) => {
                    for adjacent_coord in flowing_water_coord.adjacent_squares() {
                        if self.material_grid.get(&adjacent_coord).unwrap().is_sand() {
                            self.material_grid.insert(adjacent_coord, Material::Water(WaterType::Flowing));
                            change_happened = true;
                        }
                    }
                },
                _ => panic!("Cannot decipher square")
            }
        }

        if self.settle_water() {
            change_happened = true;
        }

        change_happened
    }

    fn can_settle(&self, coord: Coordinate, direction: Direction) -> bool {
        let mut current_coord = coord.square_to_the(direction);

        loop {
            match self.material_grid.get(&current_coord) {
                Some(Material::Water(_)) => {
                    // Must be able to sit still on top of all points
                    match self.material_grid.get(&current_coord.square_below()) {
                        Some(Material::Clay) | Some(Material::Water(WaterType::Still)) => {},
                        _ => { return false; },
                    }
                    current_coord = current_coord.square_to_the(direction);
                },
                Some(Material::Sand) | None => {
                    return false;
                },
                Some(Material::Clay) => {
                    return true;
                },
                square => panic!("Cannot decipher square when going {:?} from {}; found {:?}", direction, coord, square)
            }
        }
    }

    fn settle_water(&mut self) -> bool {
        let mut something_changed: bool = false;
        let flowing_water_coordinates: Vec<_> = self.material_grid.iter().filter(|(_, material)| material.is_flowing_water()).map(|(&c, _)| c).collect();

        for flowing_water_coord in flowing_water_coordinates {
            if let Some(material) = self.material_grid.get(&flowing_water_coord.square_below()) {
                if !material.can_stay_on() {
                    continue;
                }
            }

            if self.can_settle(flowing_water_coord, Direction::Left) && self.can_settle(flowing_water_coord, Direction::Right) {
                self.material_grid.insert(flowing_water_coord, Material::Water(WaterType::Still));
                something_changed = true;
            }
        }

        something_changed
    }

    fn total_water_count(&self) -> usize {
        self.material_grid.iter()
            .filter(|(&c, material)| material.is_water() && self.min_coord <= c && self.max_coord >= c)
            .count()
    }

    fn total_still_water_count(&self) -> usize {
        self.material_grid.iter()
            .filter(|(&c, material)| material.is_still_water() && self.min_coord <= c && self.max_coord >= c)
            .count()
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
    let mut underground = Underground::new(sand_locations);

    underground.overflow_spring();

    loop {
        let change_happened = underground.make_water_flow();
        if !change_happened {
            break;
        }
    }

    print!("{}", underground);

    Ok(underground.total_water_count())
}

pub fn q2(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let sand_locations: Vec<String> = f_contents.lines().map(|x: &str| {
        x.trim().to_string()
    }).collect();

    _q2(sand_locations).unwrap()
}

fn _q2(sand_locations: Vec<String>) -> Result<usize> {
    let mut underground = Underground::new(sand_locations);

    underground.overflow_spring();

    loop {
        let change_happened = underground.make_water_flow();
        if !change_happened {
            break;
        }
    }

    print!("{}", underground);

    Ok(underground.total_still_water_count())
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

    #[test]
    fn q2_test() {
        assert_eq!(
            _q2(vec![
                "x=495, y=2..7".to_string(),
                "y=7, x=495..501".to_string(),
                "x=501, y=3..7".to_string(),
                "x=498, y=2..4".to_string(),
                "x=506, y=1..2".to_string(),
                "x=498, y=10..13".to_string(),
                "x=504, y=10..13".to_string(),
                "y=13, x=498..504".to_string(),
            ]).unwrap(),
            29
        );
    }
}
