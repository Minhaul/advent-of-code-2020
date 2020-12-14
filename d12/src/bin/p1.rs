use file_reader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
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
    // Position is (East/West, North/South) with West and South being negative values
    pos: (i32, i32),
    heading: Direction,
}

impl Ship {
    fn new(instructions: Vec<Instruction>) -> Ship {
        Ship {
            instructions: instructions,
            instr_idx: 0,
            pos: (0, 0),
            heading: Direction::East,
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
            Action::North => self.pos.1 += inst.value as i32,
            Action::South => self.pos.1 -= inst.value as i32,
            Action::East => self.pos.0 += inst.value as i32,
            Action::West => self.pos.0 -= inst.value as i32,
            Action::Left => {
                let to_turn = inst.value % 360;
                let new_dir_deg = (direction_to_degrees(self.heading) + to_turn) % 360;
                self.heading = degrees_to_direction(new_dir_deg);
            },
            Action::Right => {
                let to_turn = 360 - (inst.value % 360);
                let new_dir_deg = (direction_to_degrees(self.heading) + to_turn) % 360;
                self.heading = degrees_to_direction(new_dir_deg);
            },
            Action::Forward => match self.heading {
                Direction::North => self.pos.1 += inst.value as i32,
                Direction::South => self.pos.1 -= inst.value as i32,
                Direction::East => self.pos.0 += inst.value as i32,
                Direction::West => self.pos.0 -= inst.value as i32,
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

fn direction_to_degrees(direction: Direction) -> u32 {
    match direction {
        Direction::North => 90,
        Direction::South => 270,
        Direction::East => 0,
        Direction::West => 180,
    }
}

fn degrees_to_direction(degrees: u32) -> Direction {
    match degrees {
        90 => Direction::North,
        270 => Direction::South,
        0 => Direction::East,
        180 => Direction::West,
        _ => panic!("Invalid degrees input!"),
    }
}
