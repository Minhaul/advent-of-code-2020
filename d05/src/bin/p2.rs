use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input.txt";

const ROW_MAX: u32 = 127;
const COL_MAX: u32 = 7;

const NUM_ROW_CHARS: usize = 7;

fn main() {
    let input_str = match file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let input: Vec<u32> =
        input_str.into_iter().map(process_input).collect();

    let mut missing: Vec<u32> = Vec::new();
    for x in 0..((ROW_MAX << 3) + COL_MAX) {
        if input.iter().find(|&&y| y == x) == None {
            missing.push(x);
        }
    }
    missing.sort();

    let mut result = 0;
    for (i, x) in missing.iter().enumerate().skip(1) {
        if i == missing.len() - 1 {
            break;
        }

        match (missing.iter().nth(i - 1), missing.iter().nth(i + 1)) {
            (Some(p), Some(n)) => {
                if *p != x - 1 && *n != x + 1 {
                    result = *x;
                    break;
                }
            },
            _ => println!("Bad input"),
        }
    }

    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn process_input(input: String) -> u32 {
    let rows = &input[0..NUM_ROW_CHARS];
    let cols = &input[NUM_ROW_CHARS..];

    let mut row_max = ROW_MAX;
    let mut row_min = 0;
    let mut col_max = COL_MAX;
    let mut col_min = 0;

    for c in rows.chars() {
        match c {
            'F' => row_max = (row_min + row_max - 1) / 2,
            'B' => row_min = (row_min + row_max + 1) / 2,
            _ => println!("Bad input"),
        };
    }
    for c in cols.chars() {
        match c {
            'L' => col_max = (col_min + col_max - 1) / 2,
            'R' => col_min = (col_min + col_max + 1) / 2,
            _ => println!("Bad input"),
        };
    }

    let row = row_max;
    let col = col_max;

    (row << 3) + col
}