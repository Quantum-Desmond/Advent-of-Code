use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result;

use std::collections::HashMap;

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

    let mut register: Register = [1, 0, 0, 0, 0, 0];

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

    let mut count: usize = 1;
    const ONE_BILLION: usize = 1_000_000_000;

    while let Some(instruction) = instructions.get(instruction_ptr) {
        register[instruction_ptr_idx] = instruction_ptr;

        opcode_fn_map.get(&instruction.opcode_name).ok_or("Cannot find opcode from name")?(instruction.instruction_set, &mut register);

        if instruction_ptr > 30 {
            println!("{:?}", register);
        }

        println!("At instruction {}, register = {:?}", instruction_ptr, register);
        if count > 30 {
            break;
        }

        instruction_ptr = register[instruction_ptr_idx];
        instruction_ptr += 1;

        if count == ONE_BILLION {
            println!("run one billion operations");
            count = 0;
        }

        count += 1;
    }

    println!("{} operations performed", count);

    Ok(register[0])
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
