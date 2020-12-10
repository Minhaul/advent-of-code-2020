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

    let result = num_bags_in(START_BAG, &input);
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

fn num_bags_in(bag_name: &str, rule_list: &Vec<BagHoldings>) -> u32 {
    let mut bag_rule = None;
    for bh in rule_list.iter() {
        if &bh.bag == bag_name {
            bag_rule = Some(bh);
            break;
        }
    }

    let mut result = 0;
    if let Some(br) = bag_rule {
        if let Some(ref v) = br.contains {
            for val in v.iter() {
                result += val.0 * (1 + num_bags_in(&val.1, rule_list));
            }
        }
    }

    result
}
