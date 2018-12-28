use std::io;
use std::fs::File;
use std::io::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

use self::Direction::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn dir_to_char(dir: Direction) -> char {
    match dir {
        Up => '^',
        Down => 'v',
        Left => '<',
        Right => '>'
    }
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Up),
        'v' => Some(Down),
        '<' => Some(Left),
        '>' => Some(Right),
        _ => None
    }
}

const ROTATIONS: [Direction; 4] = [Up, Left, Down, Right];
fn new_dir_from_old(dir: Direction, last_junction_dir: Direction) -> Direction {
    let offset: i32 = match last_junction_dir {
        Up => 0,
        Left => 1,
        Right => -1,
        _ => panic!()
    };

    let idx = ROTATIONS.iter().position(|d| *d == dir).unwrap();
    ROTATIONS[(idx as i32 + 4 + offset) as usize % 4]
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dir: Direction,
    last_junction_dir: Direction
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, dir: Direction) -> Cart {
        Cart {
            id,
            x,
            y,
            dir,
            last_junction_dir: Right
        }
    }

    fn rotate_last(&mut self) {
        self.last_junction_dir = match self.last_junction_dir {
            Right => Left,
            Left => Up,
            Up => Right,
            Down => panic!()
        };
    }
}

fn is_arrow(c: char) -> bool {
    char_to_dir(c).is_some()
}

fn x_change(dir: &Direction) -> i32 {
    match dir {
        Right => 1,
        Left => -1,
        _ => 0
    }
}

fn y_change(dir: &Direction) -> i32 {
    match dir {
        Down => 1,
        Up => -1,
        _ => 0
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn new_dir(cart: &mut Cart, grid: &Vec<Vec<char>>) -> Result<Direction, (usize, usize)> {
    let c: char = grid[cart.y][cart.x];

    if is_arrow(c) {
        return Err((cart.x, cart.y));
    }

    match c {
        '/' => match cart.dir {
            Right => Ok(Up),
            Up => Ok(Right),
            Left => Ok(Down),
            Down => Ok(Left)
        },
        '\\' => match cart.dir {
            Right => Ok(Down),
            Down => Ok(Right),
            Left => Ok(Up),
            Up => Ok(Left)
        },
        '-' | '|' => Ok(cart.dir),
        '+' => {
            cart.rotate_last();
            let result = new_dir_from_old(cart.dir, cart.last_junction_dir);
            Ok(result)
        },
        _ => panic!()
    }
}

fn sanitise_track(c: char) -> char {
    match c {
        '^' | 'v' => '|',
        '<' | '>' => '-',
        ' ' => panic!(),
        cc => cc
    }
}

pub fn q1(fname: String) -> (usize, usize) {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let text_lines: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let orig_char_grid: Vec<Vec<char>> = text_lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let grid_size: usize = 150;

    let mut cart_list: Vec<Cart> = vec![];
    for y in 0..grid_size { for x in 0..grid_size {
        if let Some(dir) = char_to_dir(orig_char_grid[y][x]) {
            let id = cart_list.len();
            cart_list.push(Cart::new(id, x, y, dir));
        }
    }}

    let mut new_char_grid = orig_char_grid.clone();
    let mut tick_count: usize = 0;

    // print_grid(&new_char_grid);
    // pause();

    loop {
        cart_list.sort_by_key(|cart| (cart.y, cart.x));

        for mut cart in cart_list.iter_mut() {
            // move cart
            let orig_pos: (usize, usize) = (cart.x, cart.y);
            cart.x = (cart.x as i32 + x_change(&cart.dir)) as usize;
            cart.y = (cart.y as i32 + y_change(&cart.dir)) as usize;

            cart.dir = match new_dir(&mut cart, &new_char_grid) {
                Ok(dir) => dir,
                Err((_x, _y)) => {
                    println!("Tick count = {}", tick_count);
                    return (cart.x, cart.y);
                }
            };
            new_char_grid[cart.y][cart.x] = dir_to_char(cart.dir);
            new_char_grid[orig_pos.1][orig_pos.0] = sanitise_track(
                orig_char_grid[orig_pos.1][orig_pos.0]
            );

        }

        // print_grid(&new_char_grid);
        // pause();

        tick_count += 1;
    }
}

pub fn q2(fname: String) -> (usize, usize) {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let text_lines: Vec<_> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let orig_char_grid: Vec<Vec<char>> = text_lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let grid_size: usize = 150;

    let mut cart_list: Vec<Cart> = vec![];
    for y in 0..grid_size { for x in 0..grid_size {
        if let Some(dir) = char_to_dir(orig_char_grid[y][x]) {
            let id = cart_list.len();
            cart_list.push(Cart::new(id, x, y, dir));
        }
    }}

    let mut new_char_grid = orig_char_grid.clone();
    let mut tick_count: usize = 0;

    let mut broken_carts: HashSet<usize> = HashSet::new();


    loop {
        cart_list.sort_by_key(|cart| (cart.y, cart.x));

        for c in cart_list.iter() {
            let cart = c.clone();
            if broken_carts.contains(&cart.id) {
                continue;
            }
            // move cart
            let orig_pos: (usize, usize) = (cart.x, cart.y);
            cart.x = (cart.x as i32 + x_change(&cart.dir)) as usize;
            cart.y = (cart.y as i32 + y_change(&cart.dir)) as usize;

            cart.dir = match new_dir(&mut cart, &new_char_grid) {
                Ok(dir) => dir,
                Err(id) => {
                    // add carts which are on dodgy space to broken_carts
                    broken_carts.insert(cart.id);
                    broken_carts.insert(id);

                    println!("Tick count = {}", tick_count);
                    continue;
                }
            };
            new_char_grid[cart.y][cart.x] = dir_to_char(cart.dir);
            new_char_grid[orig_pos.1][orig_pos.0] = sanitise_track(
                orig_char_grid[orig_pos.1][orig_pos.0]
            );

        }

        tick_count += 1;
    }
}
