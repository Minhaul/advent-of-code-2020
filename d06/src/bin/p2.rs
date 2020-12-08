use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };

    let mut input_str_f: Vec<String> = Vec::new();
    let mut input_idx = 0;
    for line in input_str.into_iter() {
        if input_str_f.len() == input_idx {
            input_str_f.push(line);
        } else if line == "" {
            input_idx += 1;
        } else {
            input_str_f[input_idx].push_str(&line);
        }
    }

    let input: Vec<Vec<char>> =
        input_str_f.into_iter().map(process_input).collect();

    let result = input.into_iter().fold(0, |acc, x| acc + x.len());
    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn process_input(input: String) -> Vec<char> {
    let mut input_vec: Vec<char> = input.chars().collect();
    input_vec.sort();
    input_vec.dedup();
    input_vec
}

