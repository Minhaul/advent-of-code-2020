use std::fmt;

use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const MIN_OCCUPIED_TO_EMPTY: u32 = 5;
const MAX_OCCUPIED_TO_FILL: u32 = 0;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

#[derive(Copy, Clone, PartialEq)]
enum SeatFill {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Debug for SeatFill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Can't use the char constants as string literals, oh well
            SeatFill::Empty => write!(f, "L"),
            SeatFill::Occupied => write!(f, "#"),
            SeatFill::Floor => write!(f, "."),
        }
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
        let line = input.get(i).unwrap();
        for j in 0..line.len() {
            let current_seat = line.get(j).unwrap();
            if *current_seat == SeatFill::Floor {
                continue;
            }

            let mut n_occupied = 0;
            for sx in -1..=1 {
                for sy in -1..=1 {
                    if sx == 0 && sy == 0 {
                        continue;
                    }

                    if occupied_seat_visible(&input, (j, i), (sx, sy)) {
                        n_occupied += 1;
                    }
                }
            }


            if (*current_seat == SeatFill::Empty && n_occupied <= MAX_OCCUPIED_TO_FILL) ||
                *current_seat == SeatFill::Occupied && n_occupied >= MIN_OCCUPIED_TO_EMPTY {
                seats_to_change.push((i, j));
            }
        }
    }

    seats_to_change
}

fn occupied_seat_visible(input: &Vec<Vec<SeatFill>>,
    start: (usize, usize), slope: (i32, i32)) -> bool {
    let (start_x, start_y) = start;
    let (slope_x, slope_y) = slope;

    let new_x: i32 = start_x as i32 + slope_x;
    if new_x < 0 || new_x >= input.get(start_y).unwrap().len() as i32 {
        return false;
    }

    let new_y: i32 = start_y as i32 + slope_y;
    if new_y < 0 || new_y >= input.len() as i32 {
        return false;
    }

    let unew_x: usize = new_x as usize;
    let unew_y: usize = new_y as usize;

    let curr_seat = *input.get(unew_y).unwrap().get(unew_x).unwrap();
    if curr_seat == SeatFill::Occupied {
        return true;
    } else if curr_seat == SeatFill::Empty {
        return false;
    } else {
        return occupied_seat_visible(input, (unew_x, unew_y), (slope_x, slope_y));
    }
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
