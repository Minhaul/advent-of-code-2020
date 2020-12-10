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
    input.push(input.last().unwrap() + 3);

    let mut curr_lvl = 0;
    let mut num_1_diffs = 0;

    let mut result: u64 = 1;
    for jlvl in input.iter() {
        match jlvl - curr_lvl {
            1 => num_1_diffs += 1,
            3 => {
                // Just to make visualization easier, working off the number of consecutive
                // numbers with a difference of 1 between them rather than the number of
                // consecutive differences of 1.
                let consecutive_nums = num_1_diffs + 1;
                if 3 <= consecutive_nums && consecutive_nums < 5 {
                    // At 3 numbers in a row with a difference of 1 between them,
                    // the middle numbers are optional (hence -2 to remove the end numbers)
                    result *= 2_u64.pow(consecutive_nums - 2);
                } else if consecutive_nums >= 5 {
                    // From 5 numbers in a row onwards, not all middle numbers are optional
                    // since we can't have more than 3 numbers in a row missing, so
                    // subtract out all the possible combinations of missing numbers that
                    // have at least 3 missing in a row. This is effectively the same as finding
                    // all combinations of numbers missing with 2 fewer numbers (since one spot of
                    // 3 missing numbers is just one effective number missing in this case that eats
                    // up two extra numbers.
                    //
                    // Maybe helpful visual example:
                    //
                    // 0: end number, ignored
                    // x: one slot of the set of 3 missing numbers in a row
                    //
                    // (0) (x) (x) (x) ( ) ( ) ( ) (0)
                    //
                    // So the three x's are one spot that eats up 3 numbers so it counts as one
                    // number effectively in the count. Note that it doesn't really matter where
                    // our theoretical spot of 3 numbers are taken out since we really care about
                    // how many numbers can be there or missing while also having our 3-in-a-row
                    // numbers missing
                    //
                    // Finally we have to subtract one from that number since it also counts the
                    // scenario where we don't actually take out any numbers
                    result *= 2_u64.pow(consecutive_nums - 2) -
                        (2_u64.pow(consecutive_nums - 4) - 1);
                }

                num_1_diffs = 0;
            },
            _ => panic!("Invalid joltage difference!"),
        }

        curr_lvl = *jlvl;
    }

    println!("{:?}", result);
}
