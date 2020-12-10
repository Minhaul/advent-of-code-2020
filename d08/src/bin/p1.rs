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
    let instructions: Vec<Instruction> =
        input_str.into_iter().map(Emulator::str_to_instructions).collect();

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
                Some(_) => break acc,
                None => *used_instructions.iter_mut().nth(pc).unwrap() = Some(()),
            },
            None => panic!("Instruction out of bounds!"),
        }

        if pc >= used_instructions.len() {
            break acc;
        }
    };

    println!("{:?}", result);
}
