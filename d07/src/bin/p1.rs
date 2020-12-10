use d07::*;
use file_reader;

const INPUT_FILENAME: &str = "input.txt";

const START_BAG: &str = "shiny gold";

fn main() {
    let input_str = match file_reader::file_to_vec(INPUT_FILENAME) {
        Err(_) => {
            println!("Couldn't turn file into vec!");
            return;
        },
        Ok(v) => v,
    };
    let input: Vec<BagHoldings> =
        input_str.into_iter().map(str_to_bag_holdings).collect();

    // Start searching for which bags are valid to start with
    let mut valid_bags: Vec<&str> = Vec::new();
    let mut bags_searched = 0;
    loop {
        let mut search_bags: Vec<&str> = Vec::new();
        let start_valid_bags = valid_bags.len();
        if start_valid_bags == 0 {
            // Just starting, only search for the start bag
            search_bags.push(START_BAG);
        } else {
            for vb in valid_bags.iter().skip(bags_searched) {
                search_bags.push(vb);
            }
        }

        // Add new bags to the valid bags list
        let mut new_valid_bags = false;
        for bh in input.iter() {
            match &bh.contains {
                Some(contains) => {
                    for sb in search_bags.iter() {
                        for cb in contains.iter() {
                            if sb == &cb.1 {
                                valid_bags.push(&bh.bag);
                                new_valid_bags = true;
                                break;
                            }
                        }
                    }
                },
                None => continue,
            };
        }


        if !new_valid_bags {
            break;
        }

        bags_searched = start_valid_bags;
    };

    valid_bags.sort();
    valid_bags.dedup();

    let result = valid_bags.len();
    println!("{:?}", result);
}

fn str_to_bag_holdings(input: String) -> BagHoldings {
    let mut bh = BagHoldings::new();

    let bag = &input[0..input.find("bags").unwrap() - 1];
    bh.bag = String::from(bag);

    let contain_idx = input.find("contain").unwrap() + "contain ".len();
    if &input[contain_idx..contain_idx + 2] != "no" {
        // This bag contains other bags
        bh.contains = Some(Vec::new());
        let mut contain_str = &input[contain_idx..];
        loop {
            let space_idx = contain_str.find(" ").unwrap();
            let bag_idx = contain_str.find("bag").unwrap();

            let num: u32 = contain_str[0..space_idx].parse().unwrap();
            let bag = &contain_str[space_idx + 1..bag_idx - 1];
            if let Some(ref mut v) = bh.contains {
                v.push((num, String::from(bag)));
            }

            match contain_str.find(",") {
                Some(i) => contain_str = &contain_str[i + 2..],
                None => break,
            };
        }
    }

    bh
}
