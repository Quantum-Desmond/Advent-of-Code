use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;

use std::collections::{HashMap, HashSet};

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
    fn new(input_1: usize, input_2: usize, output: usize) -> InstructionSet {
        InstructionSet {
            input_1,
            input_2,
            output,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Instruction {
    opcode_name: String,
    instruction_set: InstructionSet,
}

impl Instruction {
    fn new(instruction_vec: Vec<String>) -> Result<Instruction> {
        if instruction_vec.len() != 4 {
            panic!();
        }

        Ok(Instruction {
            opcode_name: instruction_vec[0].clone(),
            instruction_set: InstructionSet::new(
                instruction_vec[1].parse()?,
                instruction_vec[2].parse()?,
                instruction_vec[3].parse()?,
            )
        })
    }
}

type Register = [usize; 6];

type Opcode = dyn Fn(InstructionSet, &mut Register);

fn addr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] + register[instruction_set.input_2];
}

fn addi(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] + instruction_set.input_2;
}

fn mulr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] * register[instruction_set.input_2];
}

fn muli(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] * instruction_set.input_2;
}

fn banr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] & register[instruction_set.input_2];
}

fn bani(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] & instruction_set.input_2;
}

fn borr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] | register[instruction_set.input_2];
}

fn bori(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1] | instruction_set.input_2;
}

fn setr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = register[instruction_set.input_1];
}

fn seti(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = instruction_set.input_1;
}

fn gtir(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if instruction_set.input_1 > register[instruction_set.input_2] {
        1
    } else { 0 };
}

fn gtri(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if register[instruction_set.input_1] > instruction_set.input_2 {
        1
    } else { 0 };
}

fn gtrr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if register[instruction_set.input_1] > register[instruction_set.input_2] {
        1
    } else { 0 };
}

fn eqir(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if instruction_set.input_1 == register[instruction_set.input_2] {
        1
    } else { 0 };
}

fn eqri(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if register[instruction_set.input_1] == instruction_set.input_2 {
        1
    } else { 0 };
}

fn eqrr(instruction_set: InstructionSet, register: &mut Register) {
    register[instruction_set.output] = if register[instruction_set.input_1] == register[instruction_set.input_2] {
        1
    } else { 0 };
}

fn create_opcode_hash_map() -> HashMap<String, &'static Opcode> {
    let mut opcodes: HashMap<String, &Opcode> = HashMap::new();
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

pub fn q1(fname: String) -> usize {
    let mut f = File::open(fname).expect("File not found");
    let mut f_contents = String::new();

    f.read_to_string(&mut f_contents).expect("Couldn't find file");
    let input_list: Vec<String> = f_contents.lines().map(|x: &str| {
        x.trim().to_string()
    }).collect();

    _q1(input_list).unwrap()
}

fn _q1(instruction_list: Vec<String>) -> Result<usize> {
    let opcode_fn_map: HashMap<String, &Opcode> = create_opcode_hash_map();

    let initial_register: Register = [0, 0, 0, 0, 0, 0];

    let mut instruction_ptr: usize = 0;
    if !instruction_list[0].starts_with("#ip ") {
        return err!("First line does not match instruction_ptr format");
    }
    let instruction_ptr_idx: usize = instruction_list[0].split(' ')
        .nth(1)
        .ok_or("Cannot find number".to_string())?
        .parse()?;

    let instructions: Vec<Instruction> = instruction_list[1..]
        .into_iter()
        .map(|s| Instruction::new(s.split(' ').map(|ss| ss.to_string()).collect()).unwrap())
        .collect();

// s   let mut count: usize = 1;
    const ONE_MILLION: usize = 100;

    let mut register_guess: usize = 0;
    loop {
        let mut register: Register = initial_register;
        register[0] = register_guess;
        let mut r3_history: HashSet<usize> = HashSet::new();
        let mut repeated_history = false;
        let mut last_value: usize = 0;
        while let Some(instruction) = instructions.get(instruction_ptr) {
            register[instruction_ptr_idx] = instruction_ptr;

            match instruction_ptr {
                17 => {
                    register[4] = register[2] / 256;
                    register[instruction_ptr_idx] = 25;
                },
                28 => {
                    if r3_history.contains(&register[3]) {
                        println!("{}", last_value);
                        pause();
                        repeated_history = true;
                        break;
                    }
                    last_value = register[3];
                    r3_history.insert(register[3]);
                    opcode_fn_map.get(&instruction.opcode_name).ok_or("Cannot find opcode from name")?(instruction.instruction_set, &mut register);
                },
                _ => {
                    opcode_fn_map.get(&instruction.opcode_name).ok_or("Cannot find opcode from name")?(instruction.instruction_set, &mut register);
                }
            }

            instruction_ptr = register[instruction_ptr_idx];
            instruction_ptr += 1;

            // count += 1;
        }
        if !repeated_history {
            println!("Escaped without looping!");
            let min_value: usize = *r3_history.iter().min().ok_or("No values in set")?;
            println!("Minimum value of r3 values = {}", min_value);
            return Ok(register_guess);
        }
        let min_value: usize = *r3_history.iter().min().ok_or("No values in set")?;
        println!("Minimum value of r3 values = {}", min_value);
        // count = 0;
        register_guess += 1;
        if register_guess % ONE_MILLION == 0 {
            println!("Worked through ten thousand tries!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_test() {
        let instruction_list: Vec<String> = "
        #ip 0
        seti 5 0 1
        seti 6 0 2
        addi 0 1 0
        addr 1 2 3
        setr 1 0 0
        seti 8 0 4
        seti 9 0 5".trim().lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(
            _q1(instruction_list).unwrap(), 6
        );
    }

}
