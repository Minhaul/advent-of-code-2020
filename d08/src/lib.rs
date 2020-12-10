#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Operation,
    arg: i32,
}

#[derive(Debug)]
pub struct Emulator<'a> {
    code: &'a Vec<Instruction>,
    pc: usize,
    acc: i32
}

impl<'a> Emulator<'a> {
    pub fn new(code: &Vec<Instruction>) -> Emulator {
        Emulator {
            code: code,
            pc: 0,
            acc: 0,
        }
    }

    pub fn str_to_instructions(input: String) -> Instruction {
        let space_idx = input.find(" ").unwrap();
        let op = match &input[0..space_idx] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!("Invalid operation!"),
        };

        let val: i32 = match input[space_idx + 2..].parse() {
            Ok(v) => v,
            Err(_) => panic!("Invalid argument value!"),
        };
        let arg = match input.chars().nth(space_idx + 1).unwrap() {
            '+' => val,
            '-' => -1 * val,
            _ => panic!("Invalid argument sign!"),
        };

        Instruction {
            op: op,
            arg: arg,
        }
    }

    // Runs one instruction, returns (pc, acc)
    pub fn run_instruction(&mut self) -> (usize, i32) {
        let inst = match self.code.get(self.pc) {
            Some(i) => i,
            None => panic!("Instruction out of bounds!"),
        };

        match inst.op {
            Operation::Acc => {
                self.acc += inst.arg;
                self.pc += 1;
            },
            Operation::Jmp => self.pc = add(self.pc, inst.arg),
            Operation::Nop => self.pc += 1,
        };

        (self.pc, self.acc)
    }
}


fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as usize
    } else {
        u + i as usize
    }
}
