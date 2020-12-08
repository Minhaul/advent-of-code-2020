use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const ROW_MAX: u32 = 127;
const COL_MAX: u32 = 7;

const NUM_ROW_CHARS: usize = 7;

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let seat_ids: Vec<u32> =
        input_str.into_iter().map(process_input).collect();
    let missing: Vec<u32> = find_missing_ids(&seat_ids, ROW_MAX, COL_MAX);

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

fn find_missing_ids(seat_ids: &Vec<u32>, max_row: u32, max_col: u32) -> Vec<u32> {
    let mut missing: Vec<u32> = Vec::new();
    for x in 0..((max_row << 3) + max_col) {
        if seat_ids.iter().find(|&&y| y == x) == None {
            missing.push(x);
        }
    }
    missing.sort();

    missing
}
