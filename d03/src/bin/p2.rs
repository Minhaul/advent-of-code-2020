use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const TREE: char = '#';

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
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
