extern crate regex;
extern crate itertools;

use std::time::Instant;

mod aoc_problems;

fn main() {
    let now = Instant::now();
    let result = aoc_problems::day_22::q2(7305, 13, 734);
    let elapsed = now.elapsed();
    println!("Answer: {:?}", result);
    println!("Elapsed time: {:?}", elapsed);
}
