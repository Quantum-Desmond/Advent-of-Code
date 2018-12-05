use std::time::Instant;

mod aoc_problems;

fn main() {
    let now = Instant::now();
    let result = aoc_problems::day_5::q2("inputs/day5.txt".to_string());
    let elapsed = now.elapsed();
    println!("Answer: {}", result);
    println!("Elapsed time: {:?}", elapsed);
}
