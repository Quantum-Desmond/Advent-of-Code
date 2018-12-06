use std::time::Instant;

mod aoc_problems;

fn main() {
    let now = Instant::now();
    let result = aoc_problems::day_6::q2("inputs/day6.txt".to_string());
    let elapsed = now.elapsed();
    println!("Answer: {}", result);
    println!("Elapsed time: {:?}", elapsed);
}
