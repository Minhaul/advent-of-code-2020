use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const VAL_TO_SUM_TO: u64 = 1492208709;

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let input: Vec<u64> =
        input_str.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();

    let mut result = 0;
    for i in 0..input.len() {
        if let Some(idx) = do_values_sum_to(VAL_TO_SUM_TO, &input, i) {
            let mut values: Vec<u64> = Vec::new();
            for val in &input[i..=idx] {
                values.push(*val);
            }
            values.sort();
            result = values.first().unwrap() + values.last().unwrap();
            break;
        }
    }

    println!("{:?}", result);
}

// Returns the last index in the given value vec that sums to the given value
fn do_values_sum_to(to_sum_to: u64, values: &Vec<u64>, start_idx: usize) -> Option<usize> {
    let mut result = None;
    let mut acc = 0;
    for (i, v) in values.iter().enumerate().skip(start_idx) {
        acc += *v;

        if acc == to_sum_to {
            result = Some(i);
            break;
        }

        if acc > to_sum_to {
            break;
        }
    }

    result
}
