use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct PwData {
    min: usize,
    max: usize,
    letter: char,
    pw: String,
}

fn main() {
    let input_str = match file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let input: Vec<PwData> =
        input_str.into_iter().map(process_input).collect();
    let result = input.into_iter().map(validate_pw).filter_map(io::Result::ok).collect::<Vec<()>>().len();
    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn process_input(input: String) -> PwData {
    // input string looks like: <u32>-<u32> <char>: <String>
    let min: usize = input[0..input.find('-').unwrap()].parse::<usize>().unwrap();
    let max: usize = input[input.find('-').unwrap() + 1..input.find(' ').unwrap()].parse::<usize>().unwrap();
    let letter: char = input.chars().nth(input.find(' ').unwrap() + 1).unwrap();
    let pw: String = String::from(&input[input.find(": ").unwrap() + 2..input.len()]);

    PwData { min, max, letter, pw }
}

fn validate_pw(pw_data: PwData) -> io::Result<()> {
    let num_letter = pw_data.pw.matches(pw_data.letter).collect::<Vec<&str>>().len();
    if pw_data.min <= num_letter && num_letter <= pw_data.max {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "invalid pw"))
    }
}
