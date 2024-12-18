use itertools::Itertools;
use regex::Regex;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn parse(vec: Vec<&str>) -> VM {
    let register_re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
    let register_lines: Vec<_> = vec.clone().into_iter().take(3).collect();
    let mut vm = VM::default();
    for (idx, line) in register_lines.into_iter().enumerate() {
        if let Some(captures) = register_re.captures(&line) {
            let register_value: u64 = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            match idx {
                0 => vm.a = register_value,
                1 => vm.b = register_value,
                2 => vm.c = register_value,
                _ => panic!(),
            };
        }
    }
    let line = vec.clone().into_iter().skip(4).next().unwrap();
    let raw_instructions: Vec<_> = line
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|token| token.parse().unwrap())
        .collect();
    vm.raw_instructions = raw_instructions.clone();
    let instructions = raw_instructions
        .into_iter()
        .tuples()
        .map(|(opcode, operand)| Instruction { opcode, operand })
        .collect();
    vm.instructions = instructions;
    vm
}


enum OperandType {
    Literal,
    Combo,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: u8,
}

#[derive(Default, Debug, Clone)]
pub struct VM {
    pub a: u64,
    pub b: u64,
    pub c: u64,

    pub instructions: Vec<Instruction>,
    pub raw_instructions: Vec<u8>,
    pub ip: usize,
}

impl VM {
    pub fn operand_value(&self, instruction: &Instruction) -> u64 {
        let operand_type = match instruction.opcode {
            0 => OperandType::Combo,
            1 => OperandType::Literal,
            2 => OperandType::Combo,
            3 => OperandType::Literal,
            4 => OperandType::Literal,
            5 => OperandType::Combo,
            6 => OperandType::Combo,
            7 => OperandType::Combo,
            _ => panic!(),
        };
        let operand_value: u64 = match operand_type {
            OperandType::Combo => match instruction.operand {
                0..=3 => instruction.operand as u64,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!("{:?}", &instruction),
            },
            OperandType::Literal => instruction.operand.into(),
        };
        operand_value
    }

    pub fn run_once(&mut self) -> Option<u8> {
        let instruction = self.instructions[self.ip];
        let operand_value = self.operand_value(&instruction);

        let mut output: Option<u8> = None;
        match instruction.opcode {
            0 => {
                self.a = self.a >> operand_value;
            }
            1 => {
                self.b ^= operand_value;
            }
            2 => {
                self.b = operand_value % 8;
            }
            3 => {
                if self.a != 0 {
                    if operand_value % 2 == 1 {
                        panic!();
                    }
                    self.ip = (operand_value as usize) / 2;
                    return None;
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                output = Some((operand_value % 8).try_into().unwrap());
            }
            6 => {
                self.b = self.a >> operand_value;
            }
            7 => {
                self.c = self.a >> operand_value;
            }
            _ => panic!(),
        }
        self.ip += 1;
        output
    }

    pub fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        loop {
            if self.ip >= self.instructions.len() {
                return output;
            }
            if let Some(value) = self.run_once() {
                output.push(value);
            }
        }
    }
}
