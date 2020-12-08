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

    let result = num_trees_hit(&input_str, (1, 1)) *
                 num_trees_hit(&input_str, (3, 1)) *
                 num_trees_hit(&input_str, (5, 1)) *
                 num_trees_hit(&input_str, (7, 1)) *
                 num_trees_hit(&input_str, (1, 2));

    println!("{:?}", result);
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn num_trees_hit(input: &Vec<String>, slope: (usize, usize)) -> usize {
    let max_len = input[0].len();
    let mut x = 0;
    let mut result = 0;

    for line in input.into_iter().step_by(slope.1) {
        if line.chars().nth(x).unwrap() == TREE {
            result += 1;
        }
        x += slope.0;
        if x >= max_len {
            x -= max_len;
        }
    }

    result
}
