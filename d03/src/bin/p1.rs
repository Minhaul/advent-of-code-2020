use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input.txt";

const TREE: char = '#';

fn main() {
    let input_str = match file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let max_len = input_str[0].len();
    let mut x = 0;
    let mut result = 0;

    for line in input_str.into_iter() {
        if line.chars().nth(x).unwrap() == TREE {
            result += 1;
        }
        x += 3;
        if x >= max_len {
            x -= max_len;
        }
    }

    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
