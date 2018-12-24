use std::collections::HashMap;

fn power_level(x: u32, y: u32, serial_num: u32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_num;
    power_level *= rack_id;

    (((power_level / 100) as i32) % 10) - 5
}

pub fn q1(serial_num: u32) -> (u32, u32) {
    let mut power_level_grid: HashMap<(u32, u32), i32> = HashMap::new();
    for x in 1..301 {
        for y in 1..301 {
            power_level_grid.insert((x, y), power_level(x, y, serial_num));
        }
    }

    let mut square_areas: HashMap<(u32, u32), i32> = HashMap::new();
    for x in 1..298 {
        for y in 1..298 {
            square_areas.insert(
                (x, y),
                power_level_grid[&(x, y)] +
                power_level_grid[&(x, y+1)] +
                power_level_grid[&(x, y+2)] +
                power_level_grid[&(x+1, y)] +
                power_level_grid[&(x+1, y+1)] +
                power_level_grid[&(x+1, y+2)] +
                power_level_grid[&(x+2, y)] +
                power_level_grid[&(x+2, y+1)] +
                power_level_grid[&(x+2, y+2)]
            );
        }
    }

    square_areas.iter().max_by_key(|(&_k, &v)| v).map(|(& k, & _v)| k).unwrap()
}

pub fn q2(serial_num: u32) -> (u32, u32, u32) {
    let mut power_level_grid: HashMap<(u32, u32), i32> = HashMap::new();
    for x in 1..301 {
        for y in 1..301 {
            power_level_grid.insert((x, y), power_level(x, y, serial_num));
        }
    }

    let mut square_areas: HashMap<(u32, u32, u32), i32> = HashMap::new();
    for x in 1..301 {
        for y in 1..301 {
            let max_squ_size = 301 - std::cmp::max(x, y);
            let mut sum: i32 = 0;
            for squ_size in 1..max_squ_size+1 {
                for i in 0..squ_size-1 {
                    sum += power_level_grid[&(x + squ_size-1, y + i)];
                    sum += power_level_grid[&(x + i, y + squ_size-1)];
                }
                sum += power_level_grid[&(x + squ_size-1, y + squ_size-1)];
                square_areas.insert((x, y, squ_size), sum);

                // let mut sum: i32 = 0;
                // for xx in x..x+squ_size {
                //     for yy in y..y+squ_size {
                //         match power_level_grid.get(&(xx, yy)) {
                //             Some(n) => {sum += n;},
                //             None => {
                //                 println!("Cannot find score for {}, {}", xx, yy);
                //                 println!("Square size = {}", squ_size);
                //                 panic!();
                //             }

                //         }
                //     }
                // }
                // square_areas.insert((x, y, squ_size), sum);
            }
        }
    }

    square_areas.iter().max_by_key(|(&_k, &v)| v).map(|(& k, & _v)| k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_tests() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn q2_tests() {
        assert_eq!(q2(18), (90, 269, 16));
        assert_eq!(q2(42), (232, 251, 12));
    }
}
