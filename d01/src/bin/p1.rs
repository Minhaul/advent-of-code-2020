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
    let input: Vec<u32> = input_str.into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut result: u32 = 0;
    'outer: for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i] + input[j] == 2020 {
                result = input[i] * input[j];
                break 'outer;
            }
        }
    };
    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
