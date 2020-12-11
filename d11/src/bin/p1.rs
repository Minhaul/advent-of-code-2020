use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const MIN_OCCUPIED_TO_EMPTY: u32 = 4;
const MAX_OCCUPIED_TO_FILL: u32 = 0;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

#[derive(Debug, Copy, Clone, PartialEq)]
enum SeatFill {
    Empty,
    Occupied,
    Floor,
}

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };

    let mut input: Vec<Vec<SeatFill>> =
        input_str.into_iter().map(str_to_seat_fill_vec).collect();

    loop {
        let to_change = seats_to_shuffle(&input);
        if to_change.len() == 0 {
            break;
        }

        for (i, j) in to_change {
            let seat = input.get_mut(i).unwrap().get_mut(j).unwrap();
            *seat = match seat {
                SeatFill::Empty => SeatFill::Occupied,
                SeatFill::Occupied => SeatFill::Empty,
                _ => panic!("Can't change non-seat!"),
            };
        }
    }

    let mut result = 0;
    for line in input.iter() {
        result += num_occupied(line);
    }
    println!("{:?}", result);
}

fn str_to_seat_fill_vec(input: String) -> Vec<SeatFill> {
    let mut row: Vec<SeatFill> = Vec::new();
    for c in input.chars() {
        match c {
            EMPTY => row.push(SeatFill::Empty),
            OCCUPIED => row.push(SeatFill::Occupied),
            FLOOR => row.push(SeatFill::Floor),
            _ => panic!("Invalid input!"),
        }
    }

    row
}

// Returns a list of seats to change
fn seats_to_shuffle(input: &Vec<Vec<SeatFill>>) -> Vec<(usize, usize)> {
    let mut seats_to_change: Vec<(usize, usize)> = Vec::new();
    for i in 0..input.len() {
        let mut extra_lines_to_check: Vec<&Vec<SeatFill>> = Vec::new();
        if i == 0 {
            extra_lines_to_check.push(input.get(i + 1).unwrap());
        } else if i == input.len() - 1 {
            extra_lines_to_check.push(input.get(i - 1).unwrap());
        } else {
            extra_lines_to_check.push(input.get(i - 1).unwrap());
            extra_lines_to_check.push(input.get(i + 1).unwrap());
        }

        let line = input.get(i).unwrap();
        for j in 0..line.len() {
            let current_seat = line.get(j).unwrap();
            if *current_seat == SeatFill::Floor {
                continue;
            }

            let mut seats_to_check: Vec<SeatFill> = Vec::new();
            if j == 0 {
                seats_to_check.push(*line.get(j + 1).unwrap());
                for extra_line in extra_lines_to_check.iter() {
                    seats_to_check.push(*extra_line.get(j).unwrap());
                    seats_to_check.push(*extra_line.get(j + 1).unwrap());
                }
            } else if j == line.len() - 1 {
                seats_to_check.push(*line.get(j - 1).unwrap());
                for extra_line in extra_lines_to_check.iter() {
                    seats_to_check.push(*extra_line.get(j - 1).unwrap());
                    seats_to_check.push(*extra_line.get(j).unwrap());
                }
            } else {
                seats_to_check.push(*line.get(j - 1).unwrap());
                seats_to_check.push(*line.get(j + 1).unwrap());
                for extra_line in extra_lines_to_check.iter() {
                    seats_to_check.push(*extra_line.get(j - 1).unwrap());
                    seats_to_check.push(*extra_line.get(j).unwrap());
                    seats_to_check.push(*extra_line.get(j + 1).unwrap());
                }
            }

            let n_occupied = num_occupied(&seats_to_check);
            if (*current_seat == SeatFill::Empty && n_occupied <= MAX_OCCUPIED_TO_FILL) ||
                *current_seat == SeatFill::Occupied && n_occupied >= MIN_OCCUPIED_TO_EMPTY {
                seats_to_change.push((i, j));
            }
        }
    }

    seats_to_change
}

fn num_occupied(seats: &Vec<SeatFill>) -> u32 {
    let mut num = 0;
    for s in seats.iter() {
        if *s == SeatFill::Occupied {
            num += 1;
        }
    }

    num
}
