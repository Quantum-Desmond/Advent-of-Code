use std::cmp;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::result;
use std::usize;

use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet};

type Result<T> = result::Result<T, Box<dyn Error>>;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
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
    x: usize,
    y: usize
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn surrounding_squares(self: Coordinate) -> Vec<Coordinate> {
        let mut possible_squares = vec![
            Coordinate { x: self.x + 1, y: self.y },
            Coordinate { x: self.x, y: self.y + 1 },
        ];

        if self.x > 0 {
            possible_squares.push(Coordinate { x: self.x - 1, y: self.y });
        }

        if self.y > 0 {
            possible_squares.push(Coordinate { x: self.x, y: self.y - 1 });
        }

        possible_squares
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
            CaveType::Narrow => write!(f, "|"),
            CaveType::Wet => write!(f, "="),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum ToolState {
    ClimbingGear,
    Torch,
    Neither
}

impl ToolState {
    fn other_allowed_tool(&self, cave_type: CaveType) -> ToolState {
        match (cave_type, &self) {
            (CaveType::Rocky, ToolState::Torch) => ToolState::ClimbingGear,
            (CaveType::Rocky, ToolState::ClimbingGear) => ToolState::Torch,
            (CaveType::Wet, ToolState::ClimbingGear) => ToolState::Neither,
            (CaveType::Wet, ToolState::Neither) => ToolState::ClimbingGear,
            (CaveType::Narrow, ToolState::Torch) => ToolState::Neither,
            (CaveType::Narrow, ToolState::Neither) => ToolState::Torch,
            _ => panic!("Disallowed combination!")
        }
    }
}

impl fmt::Display for ToolState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ToolState::Torch => write!(f, "Torch"),
            ToolState::ClimbingGear => write!(f, "Climbing Gear"),
            ToolState::Neither => write!(f, "Neither"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct CaveState {
    pos: Coordinate,
    tool: ToolState
}

impl fmt::Display for CaveState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "At {} with {}", self.pos, self.tool)
    }
}

impl CaveState {
    fn new(pos: Coordinate, tool: ToolState) -> CaveState {
        CaveState { pos, tool }
    }
}

struct Caves {
    depth: usize,
    target: Coordinate,
    geologic_idx_map: BTreeMap<Coordinate, usize>,
    caves_types: BTreeMap<Coordinate, CaveType>,
    allowed_tools: HashMap<CaveType, HashSet<ToolState>>
}

struct Visit {
    point: CaveState,
    distance: usize,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Eq for Visit {}

impl Caves {
    fn new(depth: usize, target: Coordinate) -> Caves {
        use self::ToolState::*;

        let mut allowed_tools: HashMap<CaveType, HashSet<ToolState>> = HashMap::new();
        allowed_tools.insert(CaveType::Rocky, set![Torch, ClimbingGear]);
        allowed_tools.insert(CaveType::Wet, set![ClimbingGear, Neither]);
        allowed_tools.insert(CaveType::Narrow, set![Torch, Neither]);

        Caves {
            depth,
            target,
            geologic_idx_map: BTreeMap::new(),
            caves_types: BTreeMap::new(),
            allowed_tools
        }
    }

    fn surrounding_squares(&self, coord: Coordinate) -> Vec<Coordinate> {
        let mut possible_squares = vec![
        ];

        if coord.x > 0 {
            possible_squares.push(Coordinate { x: coord.x - 1, y: coord.y });
        }
        if coord.y > 0 {
            possible_squares.push(Coordinate { x: coord.x, y: coord.y - 1 });
        }

        if coord.x < self.target.x + 10 {
            possible_squares.push(Coordinate { x: coord.x + 1, y: coord.y });
        }
        if coord.y < self.target.y + 10 {
            possible_squares.push(Coordinate { x: coord.x, y: coord.y + 1 });
        }

        possible_squares
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
        // Add a buffer of about 10 past the target, to help exploring
        for y in 0..self.target.y+11 {
            for x in 0..self.target.x+11 {
                let coord = Coordinate::new(x, y);
                let erosion_level = self.calculate_erosion_lvl(coord);
                self.caves_types.insert(coord, CaveType::new(erosion_level));
            }
        }
    }

    fn total_risk_level(&self) -> usize {
        self.caves_types.iter()
            .filter(|(coord, _)| (coord.x <= self.target.x) && (coord.y <= self.target.y))
            .map(|(_, cave_type)| cave_type.to_risk_lvl()).sum()
    }

    fn is_allowed_tool(&self, cave_state: CaveState) -> bool {
        let current_cave_state = match self.caves_types.get(&cave_state.pos) {
            Some(cave_state) => cave_state,
            None => panic!("Have not yet determined cave state of {}", cave_state.pos)
        };

        self.allowed_tools.get(current_cave_state).unwrap().contains(&cave_state.tool)
    }

    fn free_squares_around(&self, state: CaveState) -> Vec<(CaveState, usize)> {
        // 1 minute to move to adjacent squares
        let mut possible_states: Vec<(CaveState, usize)> = self.surrounding_squares(state.pos)
            .into_iter()
            .map(|c| CaveState::new(c, state.tool))
            .filter(|&cave_state| self.is_allowed_tool(cave_state))
            .map(|cave_state| (cave_state, 1))
            .collect();

        let current_cave_state = self.caves_types.get(&state.pos).unwrap();
        let other_allowed_tool = state.tool.other_allowed_tool(*current_cave_state);

        // 7 minutes to change tools
        possible_states.push((CaveState::new(state.pos, other_allowed_tool), 7));
        possible_states
    }

    fn fastest_path_to_target(&self) -> usize {
        let mut times: BTreeMap<CaveState, usize> = BTreeMap::new();
        let start_state: CaveState = CaveState::new(Coordinate::new(0, 0), ToolState::Torch);
        times.insert(start_state, 0);

        let search_target = CaveState::new(self.target, ToolState::Torch);

        let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
        queue.push(Visit { point: start_state, distance: 0 });

        let mut visited: BTreeSet<CaveState> = BTreeSet::new();

        while let Some(state) = queue.pop() {
            if !visited.insert(state.point) {
                continue;
            }

            if state.point == search_target {
                break;
            }

            for (neighbour, time_diff) in self.free_squares_around(state.point) {
                let new_time = time_diff + *times.get(&state.point).unwrap();
                if !times.contains_key(&neighbour) || new_time < times[&neighbour] {
                    times.insert(neighbour, new_time);
                    queue.push(Visit {
                        point: neighbour,
                        distance: new_time,
                    });
                }
            }
        }

        let final_time = *times.get(&search_target).unwrap();

        final_time
    }
}

impl fmt::Display for Caves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut previous_row_num: usize = 0;
        for (c, square) in &self.caves_types {
            if c.y != previous_row_num {
                write!(f, "\n")?;
                previous_row_num = c.y;
            }
            if *c == self.target {
                write!(f, "T")?;
                continue;
            }
            write!(f, "{}", square)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

pub fn q1(depth: usize, target_x: usize, target_y: usize) -> usize {
    let mut caves = Caves::new(depth, Coordinate{x: target_x, y: target_y});

    caves.calculate_erosion_levels();

    // print!("{}", caves);

    caves.total_risk_level()
}

pub fn q2(depth: usize, target_x: usize, target_y: usize) -> usize {
    let mut caves = Caves::new(depth, Coordinate{x: target_x, y: target_y});
    caves.calculate_erosion_levels();

    caves.fastest_path_to_target()
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

    #[test]
    fn day22_q2_test() {
        assert_eq!(
            q2(510, 10, 10),
            45
        );
    }
}
