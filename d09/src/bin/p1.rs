use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const NUM_VALID_PAST_VALUES: usize = 25;

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

    let mut past_x_values: Vec<u64> = Vec::with_capacity(NUM_VALID_PAST_VALUES);
    let mut result = 0;
    for (i, val) in input.iter().enumerate() {
        if i < past_x_values.capacity() {
            past_x_values.push(*val);
            continue;
        }

        if find_two_to_sum(*val, &past_x_values) == None {
            result = *val;
            break;
        }

        past_x_values.remove(0);
        past_x_values.push(*val);
    }

    println!("{:?}", result);
}

fn find_two_to_sum(to_sum_to: u64, values: &Vec<u64>) -> Option<(u64, u64)> {
    let mut result = None;
    'outer: for v1 in values.iter() {
        for v2 in values.iter() {
            if v1 == v2 {
                continue;
            }

            if v1 + v2 == to_sum_to {
                result = Some((*v1, *v2));
                break 'outer;
            }
        }
    }

    result
}
