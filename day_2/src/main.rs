use std::io;
use std::io::Read;


fn main() {
    let mut stdin = io::stdin();
    let mut program_string = String::new();
    stdin.read_to_string(&mut program_string)
        .expect("Failed to read file.");

    // Create program
    let program = IntcodeProgram::from(&program_string);

    for noun in 0..100 {
        for verb in 0..100 {
            if try_with(noun, verb, &program) {
                println!("{} and {}: answer {}", noun, verb, (100 * noun + verb));
                return;
            }
        }
    }
    println!("No answer!");
}

fn try_with(noun: i64, verb: i64, program: &IntcodeProgram) -> bool {
    let mut program_copy = program.clone();
    restore_gravity_assist(noun, verb, &mut program_copy);

    let mut cpu = Cpu::new();
    cpu.execute(&mut program_copy);

    return program_copy.read_at(0) == 19690720;
}

fn restore_gravity_assist(noun: i64, verb: i64, program: &mut dyn Memory) {
    program.write_at(noun, 1);
    program.write_at(verb, 2);
}

type Address = usize;
type Value = i64;
// struct Address(u64);

trait Memory {
    fn read_at(&self, address: Address) -> i64;
    fn write_at(&mut self, value: i64, address: Address);
}

enum Instruction {
    Add(Address, Address, Address),
    Mult(Address, Address, Address),
    Halt(),
}

struct Cpu {
    instruction_ptr: Address,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            instruction_ptr: 0
        }
    }

    fn execute(&mut self, memory: &mut dyn Memory) {

        loop {
            let instruction = self.get_next_instruction(memory);

            match instruction {
                Instruction::Add(param_0, param_1, res) => {
                    eprintln!("exec ADD  @{:3}: &{:3} &{:3} ->&{:3} ({:3}+{:3})",
                        self.instruction_ptr-4, param_0, param_1, res, memory.read_at(param_0),
                        memory.read_at(param_1));
                    memory.write_at(memory.read_at(param_0) + memory.read_at(param_1), res);
                },
                Instruction::Mult(param_0, param_1, res) => {
                    eprintln!("exec MULT @{:3}: &{:3} &{:3} ->&{:3} ({:3}*{:3})",
                        self.instruction_ptr-4, param_0, param_1, res, memory.read_at(param_0),
                        memory.read_at(param_1));
                    memory.write_at(memory.read_at(param_0) * memory.read_at(param_1), res);
                },
                Instruction::Halt() => {
                    eprintln!("exec HALT @{:3}:", self.instruction_ptr-4);
                    return
                },
            }
        }
    }

    fn get_next_instruction(&mut self, program: &dyn Memory) -> Instruction {
        let instruction = self.instruction_at(self.instruction_ptr, program);
        self.instruction_ptr += 4;
        instruction
    }

    fn instruction_at(&self, address: Address, program: &dyn Memory) -> Instruction {
        let opcode = program.read_at(address);
        match opcode {
            1 => {
                let param_addr_0 = program.read_at(address + 1) as Address;
                let param_addr_1 = program.read_at(address + 2) as Address;
                let result_addr = program.read_at(address + 3) as Address;
                Instruction::Add(param_addr_0, param_addr_1, result_addr)
            },
            2 => {
                let param_addr_0 = program.read_at(address + 1) as Address;
                let param_addr_1 = program.read_at(address + 2) as Address;
                let result_addr = program.read_at(address + 3) as Address;
                Instruction::Mult(param_addr_0, param_addr_1, result_addr)
            },
            99 => Instruction::Halt(),
            _ => Instruction::Halt(), // this should error instead
        }
    }
}

struct IntcodeProgram {
    raw_program: Vec<Value>,
}

impl Memory for IntcodeProgram {
    fn read_at(&self, address: Address) -> i64 {
        self.raw_program[address]
    }

    fn write_at(&mut self, value: i64, address: Address) {
        self.raw_program[address] = value;
    }
}

impl IntcodeProgram {
    fn from(s: &String) -> IntcodeProgram {
        let program_vec = s.trim()
            .split(",")
            .map(|code| code.parse().expect("Failed to parse code"))
            .collect();

        IntcodeProgram{
            raw_program: program_vec,
        }
    }

    fn from_vec(v: Vec<Value>) -> IntcodeProgram {
        IntcodeProgram{
            raw_program: v,
        }
    }

    fn len(&self) -> usize {
        self.raw_program.len()
    }
}

impl Clone for IntcodeProgram {
    fn clone(&self) -> Self {
        let mut clone = Self {
            raw_program: Vec::with_capacity(self.raw_program.len()),
        };
        for value in self.raw_program.iter() {
            clone.raw_program.push(*value);
        }
        clone
    }
}
