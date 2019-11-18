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
struct InstructionSet {
    input_1: usize,
    input_2: usize,
    output: usize
}

impl InstructionSet {
    fn new(instruction_vec: &[usize]) -> InstructionSet {
        if instruction_vec.len() != 3 {
            panic!();
        }

        InstructionSet {
            input_1: instruction_vec[0],
            input_2: instruction_vec[1],
            output: instruction_vec[2],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Instruction {
    opcode_num: usize,
    instruction_set: InstructionSet,
}

impl Instruction {
    fn new(instruction_vec: Vec<usize>) -> Instruction {
        if instruction_vec.len() != 4 {
            panic!();
        }

        Instruction {
            opcode_num: instruction_vec[0],
            instruction_set: InstructionSet::new(&instruction_vec[1..])
        }
    }
}

type Register = [usize; 4];

fn new_register(input: Vec<usize>) -> Register {
    if input.len() != 4 {
        panic!();
    }

    [input[0], input[1], input[2], input[3]]
}

type Opcode = dyn Fn(InstructionSet, Register) -> Register;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Sample {
    before: Register,
    instruction: Instruction,
    after: Register
}

impl Sample {
    fn new(before: Register, instruction: Instruction, after: Register) -> Sample {
        Sample {
            before,
            instruction,
            after
        }
    }
}

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let input_list: Vec<String> = f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let samples: Vec<Sample> = input_list
        .windows(3)
        .filter_map(|strings| {
            match strings[0].starts_with("Before: ") && strings[2].starts_with("After: ") {
                false => None,
                true => Some(Sample::new(
                    new_register(
                        strings[0]
                            .replace("Before:", "")
                            .trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace())
                            .split(", ")
                            .map(|s| s.parse().unwrap())
                            .collect()
                    ),
                    Instruction::new(
                        strings[1]
                            .trim()
                            .split(' ')
                            .into_iter()
                            .map(|s| s.parse().unwrap())
                            .collect()
                    ),
                    new_register(
                        strings[2]
                            .replace("After:", "")
                            .trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace())
                            .split(", ")
                            .map(|s| s.parse().unwrap())
                            .collect()
                    ),
                ))
            }
        })
        .collect();

    println!("Number of input samples = {}", samples.len());

    0
}


#[cfg(test)]
mod tests {
    use super::*;

}
