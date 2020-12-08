use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct PwData {
    i1: usize,
    i2: usize,
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
    let i1: usize = input[0..input.find('-').unwrap()].parse::<usize>().unwrap() - 1;
    let i2: usize = input[input.find('-').unwrap() + 1..input.find(' ').unwrap()].parse::<usize>().unwrap() - 1;
    let letter: char = input.chars().nth(input.find(' ').unwrap() + 1).unwrap();
    let pw: String = String::from(&input[input.find(": ").unwrap() + 2..input.len()]);

    PwData { i1, i2, letter, pw }
}

fn validate_pw(pw_data: PwData) -> io::Result<()> {
    let char1: char = pw_data.pw.chars().nth(pw_data.i1).unwrap();
    let char2: char = pw_data.pw.chars().nth(pw_data.i2).unwrap();
    if (char1 == pw_data.letter) ^ (char2 == pw_data.letter) {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "invalid pw"))
    }
}
