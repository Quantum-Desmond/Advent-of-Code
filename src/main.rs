use std::time::Instant;

mod aoc_problems;

fn main() {
    let now = Instant::now();
    let result = aoc_problems::day_2::q1("inputs/day2_1.txt".to_string());
    let elapsed = now.elapsed();
    println!("Answer: {}", result);
    println!("Elapsed time: {:?}", elapsed);
}
