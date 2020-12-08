use file_reader;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };

    // Vec is (number of people, answers combined)
    let mut input_str_f: Vec<(u32, String)> = Vec::new();
    let mut input_idx = 0;
    for line in input_str.into_iter() {
        if input_str_f.len() == input_idx {
            input_str_f.push((1, line));
        } else if line == "" {
            input_idx += 1;
        } else {
            input_str_f[input_idx].0 += 1;
            input_str_f[input_idx].1.push_str(&line);
        }
    }

    let input: Vec<u32> =
        input_str_f.into_iter().map(num_valid_answers).collect();

    let result = input.into_iter().fold(0, |acc, x| acc + x);
    println!("{:?}", result);
}

fn num_valid_answers(input: (u32, String)) -> u32 {
    let grp_size: u32 = input.0;
    let answers: String = input.1;

    let mut input_vec: Vec<char> = answers.chars().collect();
    input_vec.sort();

    let mut valid_answers = 0;

    let mut answer_cnt = 0;
    let mut curr_char = ' ';
    for c in input_vec.into_iter() {
        if c != curr_char {
            curr_char = c;
            answer_cnt = 1;
        } else {
            answer_cnt += 1;
        }

        if answer_cnt == grp_size {
            valid_answers += 1;
        } else if answer_cnt > grp_size {
            // Better than no error handling ...probably
            println!("BAD THINGS HAPPENING!");
        }
    };

    valid_answers
}

