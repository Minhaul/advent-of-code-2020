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
    let mut input: Vec<u32> =
        input_str.into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
    input.sort();

    let mut curr_lvl = 0;
    let mut num_diff_1 = 0;
    // Always start with 1 here since the last conversion is always 3
    let mut num_diff_3 = 1;

    for jlvl in input.iter() {
        match jlvl - curr_lvl {
            1 => num_diff_1 += 1,
            3 => num_diff_3 += 1,
            _ => panic!("Invalid joltage difference!"),
        }

        curr_lvl = *jlvl;
    }

    let result = num_diff_1 * num_diff_3;
    println!("{:?}", result);
}
