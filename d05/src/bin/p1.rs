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
    let input: Vec<u32> =
        input_str.into_iter().map(process_input).collect();
    let result = input.into_iter().max().unwrap();
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
