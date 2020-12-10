use d08::*;
use file_reader;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let mut instructions: Vec<Instruction> =
        input_str.into_iter().map(Emulator::str_to_instructions).collect();

    let mut result: i32 = 0;
    // We can't directly iterate over instructions:
    // 1. We can't take a mut reference when iterating because we pass an immutable reference
    //    to `does_terminate`, which means
    // 2. we need to do an inline replace of an op in `instructions` based on index which means
    //    we also can't have an immutable reference to instructions from a normal iter
    for i in 0..instructions.len() {
        let mut new_op;
        match instructions.get(i) {
            Some(inst) => match inst.op {
                Operation::Jmp => new_op = Operation::Nop,
                Operation::Nop => new_op = Operation::Jmp,
                _ => continue,
            },
            None => panic!("Out of bounds!"),
        };

        instructions.iter_mut().nth(i).unwrap().op = new_op;

        if let Some(acc) = does_terminate(&instructions) {
            result = acc;
            break;
        }

        if new_op == Operation::Jmp {
            new_op = Operation::Nop;
        } else {
            new_op = Operation::Jmp;
        }

        instructions.iter_mut().nth(i).unwrap().op = new_op;
    }

    println!("{:?}", result);
}

fn does_terminate(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut used_instructions: Vec<Option<()>> = Vec::with_capacity(instructions.len());
    used_instructions.push(Some(()));
    for _ in 1..instructions.len() {
        used_instructions.push(None);
    }

    let mut emulator: Emulator = Emulator::new(&instructions);
    let result = loop {
        let (pc, acc) = emulator.run_instruction();
        match used_instructions.get(pc) {
            Some(uinst) => match uinst {
                Some(_) => break None,
                None => *used_instructions.iter_mut().nth(pc).unwrap() = Some(()),
            },
            None => break Some(acc),
        }
    };

    result
}
