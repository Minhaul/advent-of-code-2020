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
