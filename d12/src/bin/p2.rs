use file_reader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: u32,
}

#[derive(Debug)]
struct Ship {
    instructions: Vec<Instruction>,
    instr_idx: usize,
    // Both are (East/West, North/South) with West and South being negative values
    pos: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    fn new(instructions: Vec<Instruction>) -> Ship {
        Ship {
            instructions: instructions,
            instr_idx: 0,
            pos: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn str_to_instructions(input: String) -> Instruction {
        let action = match input.chars().nth(0) {
            Some(c) => match c {
                'N' => Action::North,
                'S' => Action::South,
                'E' => Action::East,
                'W' => Action::West,
                'L' => Action::Left,
                'R' => Action::Right,
                'F' => Action::Forward,
                _ => panic!("Invalid operation!"),
            },
            None => panic!("Invalid input string!"),
        };

        let value: u32 = match input.get(1..) {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(_) => panic!("Invalid argument value!"),
            },
            None => panic!("Invalid input string!"),
        };

        Instruction {
            action: action,
            value: value,
        }
    }

    // Runs one instruction, returns position
    fn perform_instruction(&mut self) -> (i32, i32) {
        let inst = match self.instructions.get(self.instr_idx) {
            Some(i) => i,
            None => panic!("Out of bounds!"),
        };

        match inst.action {
            Action::North => self.waypoint.1 += inst.value as i32,
            Action::South => self.waypoint.1 -= inst.value as i32,
            Action::East => self.waypoint.0 += inst.value as i32,
            Action::West => self.waypoint.0 -= inst.value as i32,
            Action::Left | Action::Right => {
                let to_turn = if inst.action == Action::Right {
                    360 - (inst.value % 360)
                } else {
                    inst.value % 360
                };

                self.waypoint = match to_turn {
                    90 => (-1 * self.waypoint.1, self.waypoint.0),
                    180 => (-1 * self.waypoint.0, -1 * self.waypoint.1),
                    270 => (self.waypoint.1, -1 * self.waypoint.0),
                    _ => self.waypoint,
                };
            },
            Action::Forward => {
                self.pos.0 += inst.value as i32 * self.waypoint.0;
                self.pos.1 += inst.value as i32 * self.waypoint.1;
            },
        };

        self.instr_idx += 1;

        self.pos
    }
}


fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let instructions: Vec<Instruction> =
        input_str.into_iter().map(Ship::str_to_instructions).collect();
    let instr_len = instructions.len();

    let mut ship: Ship = Ship::new(instructions);
    let mut pos = (0, 0);
    for _ in 0..instr_len {
        pos = ship.perform_instruction();
    };

    let result = pos.0.abs() + pos.1.abs();
    println!("{:?}", result);
}
