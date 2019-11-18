use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

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

fn addr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] + after[instruction_set.input_2];

    after
}

fn addi(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] + instruction_set.input_2;

    after
}

fn mulr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] * after[instruction_set.input_2];

    after
}

fn muli(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] * instruction_set.input_2;

    after
}

fn banr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] & after[instruction_set.input_2];

    after
}

fn bani(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] & instruction_set.input_2;

    after
}

fn borr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] | after[instruction_set.input_2];

    after
}

fn bori(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1] | instruction_set.input_2;

    after
}

fn setr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = after[instruction_set.input_1];

    after
}

fn seti(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = instruction_set.input_1;

    after
}

fn gtir(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if instruction_set.input_1 > after[instruction_set.input_2] {
        1
    } else { 0 };

    after
}

fn gtri(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if after[instruction_set.input_1] > instruction_set.input_2 {
        1
    } else { 0 };

    after
}

fn gtrr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if after[instruction_set.input_1] > after[instruction_set.input_2] {
        1
    } else { 0 };

    after
}

fn eqir(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if instruction_set.input_1 == after[instruction_set.input_2] {
        1
    } else { 0 };

    after
}

fn eqri(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if after[instruction_set.input_1] == instruction_set.input_2 {
        1
    } else { 0 };

    after
}

fn eqrr(instruction_set: InstructionSet, before: Register) -> Register {
    let mut after = before.clone();
    after[instruction_set.output] = if after[instruction_set.input_1] == after[instruction_set.input_2] {
        1
    } else { 0 };

    after
}

fn create_opcode_string_map() -> BTreeMap<String, &'static Opcode> {
    let mut opcodes: BTreeMap<String, &Opcode> = BTreeMap::new();
    opcodes.insert("addr".to_string(), &addr);
    opcodes.insert("addi".to_string(), &addi);
    opcodes.insert("mulr".to_string(), &mulr);
    opcodes.insert("muli".to_string(), &muli);
    opcodes.insert("banr".to_string(), &banr);
    opcodes.insert("bani".to_string(), &bani);
    opcodes.insert("borr".to_string(), &borr);
    opcodes.insert("bori".to_string(), &bori);
    opcodes.insert("setr".to_string(), &setr);
    opcodes.insert("seti".to_string(), &seti);
    opcodes.insert("gtir".to_string(), &gtir);
    opcodes.insert("gtri".to_string(), &gtri);
    opcodes.insert("gtrr".to_string(), &gtrr);
    opcodes.insert("eqir".to_string(), &eqir);
    opcodes.insert("eqri".to_string(), &eqri);
    opcodes.insert("eqrr".to_string(), &eqrr);

    opcodes
}

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

    fn opcode_matches(&self, opcodes: &BTreeMap<String, &Opcode>) -> usize {
        opcodes.values().filter(|opcode| opcode(self.instruction.instruction_set, self.before) == self.after).count()
    }
}

fn generate_opcode_number_map(opcode_str_map: BTreeMap<String, &Opcode>, samples: Vec<Sample>) -> HashMap<usize, &Opcode> {
    let mut opcode_possibilities: BTreeMap<usize, HashSet<String>> = (0..16)
        .map(|n| (n, opcode_str_map.keys().map(|s| s.clone()).collect()))
        .collect();

    for sample in samples {
        let possible_opcodes: HashSet<String> = opcode_str_map
            .iter()
            .filter(|(_, opcode_fn)| opcode_fn(sample.instruction.instruction_set, sample.before) == sample.after)
            .map(|(s, _)| s.clone())
            .collect();
        println!("Number of possible opcodes = {}", possible_opcodes.len());

        let previous_guesses = match opcode_possibilities.get_mut(&sample.instruction.opcode_num) {
            Some(set) => set,
            None => panic!("{} is not in the hash map for opcode numbers", sample.instruction.opcode_num)
        };

        *previous_guesses = previous_guesses.intersection(&possible_opcodes).cloned().collect();
    }

    println!("Possibilities now {:?}", opcode_possibilities);

    let mut number_translation_map: HashMap<usize, &Opcode> = HashMap::new();

    while opcode_possibilities.len() > 0 {
        let mut decided_opcodes: VecDeque<(usize, String, &Opcode)> = opcode_possibilities
            .iter()
            .filter(|(_, string_set)| string_set.len() == 1)
            .map(|(idx, string_set)| (idx, string_set.iter().collect::<Vec<_>>()[0].clone()))
            .map(|(&idx, s)| (idx, s.clone(), *opcode_str_map.get(&s).unwrap()))
            .collect();

        if decided_opcodes.len() == 0 {
            println!("Cannot decipher which opcodes are which!: {:?}", opcode_possibilities);
            panic!("Cannot decipher which opcodes are which!");
        }

        while let Some((idx, opcode_str, opcode_fn)) = decided_opcodes.pop_front() {
            println!("{} corresponds to {}", idx, opcode_str);
            number_translation_map.insert(idx, opcode_fn);
            opcode_possibilities.remove(&idx);
            for opcode_set in opcode_possibilities.values_mut() {
                opcode_set.remove(&opcode_str);
            }
        }
    }

    number_translation_map
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

    let opcodes = create_opcode_string_map();

    samples.iter().filter(|sample| sample.opcode_matches(&opcodes) >= 3).count()
}

pub fn q2(fname: String) -> usize {
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

    let opcodes = create_opcode_string_map();
    let opcode_num_map = generate_opcode_number_map(opcodes, samples);

    let mut instruction_f = File::open("./inputs/day16_2.txt").expect("File not found");
    let mut instruction_f_contents = String::new();

    instruction_f.read_to_string(&mut instruction_f_contents).expect("Couldn't find file");
    let instruction_list: Vec<String> = instruction_f_contents.lines().map(|x: &str| {
        x.to_string()
    }).collect();

    let instruction_list: Vec<Instruction> = instruction_list.into_iter().map(|l| {
        Instruction::new(
            l.trim()
                .split(' ')
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect()
        )
    }).collect();

    println!("Instruction count = {}", instruction_list.len());

    let mut register: Register = [0, 0, 0, 0];
    for instruction in instruction_list {
        register = opcode_num_map.get(&instruction.opcode_num).unwrap()(instruction.instruction_set, register);
    }

    println!("Register is now {:?}", register);

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_test() {
        let strings = [
            "Before: [3, 2, 1, 1]".to_string(),
            "9 2 1 2".to_string(),
            "After:  [3, 2, 2, 1]".to_string(),
        ];
        let before_register = new_register(
            strings[0]
                .replace("Before:", "")
                .trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace())
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect()
        );
        let after_register = new_register(
            strings[2]
                .replace("After:", "")
                .trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace())
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect()
        );
        let sample = Sample::new(
            before_register,
            Instruction::new(
                strings[1]
                    .trim()
                    .split(' ')
                    .into_iter()
                    .map(|s| s.parse().unwrap())
                    .collect()
            ),
            after_register
        );

        let opcodes = create_opcode_string_map();
        assert_eq!(
            sample.opcode_matches(&opcodes), 3
        );
    }

}
