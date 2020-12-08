use std::io;

use d04::*;
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

    let mut input_str_f: Vec<String> = Vec::new();
    let mut input_idx = 0;
    for line in input_str.into_iter() {
        if input_str_f.len() == input_idx {
            input_str_f.push(line);
        } else if line == "" {
            input_idx += 1;
        } else {
            input_str_f[input_idx].push_str(" ");
            input_str_f[input_idx].push_str(&line);
        }
    }

    let input: Vec<PpData> =
        input_str_f.into_iter().map(process_input).collect();

    let result = input.into_iter().map(validate_pp).filter_map(io::Result::ok).collect::<Vec<()>>().len();
    println!("{:?}", result);
}

fn process_input(input: String) -> PpData {
    let mut data: PpData = PpData::new();
    for info in input.split(" ") {
        let colon_idx = info.find(':').unwrap();
        let key = &info[0..colon_idx];
        let val = &info[colon_idx + 1..];
        match key {
            "byr" => data.byr = Some(String::from(val)),
            "iyr" => data.iyr = Some(String::from(val)),
            "eyr" => data.eyr = Some(String::from(val)),
            "hgt" => data.hgt = Some(String::from(val)),
            "hcl" => data.hcl = Some(String::from(val)),
            "ecl" => data.ecl = Some(String::from(val)),
            "pid" => data.pid = Some(String::from(val)),
            "cid" => data.cid = Some(String::from(val)),
            _ => println!("BAD THINGS HAPPENING!"),
        }
    }

    data
}

fn validate_pp(data: PpData) -> io::Result<()> {
    if byr_valid(data.byr) &&
       iyr_valid(data.iyr) &&
       eyr_valid(data.eyr) &&
       hgt_valid(data.hgt) &&
       hcl_valid(data.hcl) &&
       ecl_valid(data.ecl) &&
       pid_valid(data.pid) {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "invalid pp"))
    }
}

fn byr_valid(byr: Option<String>) -> bool {
    match byr {
        Some(val) => {
            match val.parse::<u32>() {
                Ok(x) => return 1920 <= x && x <= 2002,
                Err(_) => return false,
            };
        },
        None => return false,
    };
}

fn iyr_valid(iyr: Option<String>) -> bool {
    match iyr {
        Some(val) => {
            match val.parse::<u32>() {
                Ok(x) => return 2010 <= x && x <= 2020,
                Err(_) => return false,
            };
        },
        None => return false,
    };
}

fn eyr_valid(eyr: Option<String>) -> bool {
    match eyr {
        Some(val) => {
            match val.parse::<u32>() {
                Ok(x) => return 2020 <= x && x <= 2030,
                Err(_) => return false,
            };
        },
        None => return false,
    };
}

fn hgt_valid(hgt: Option<String>) -> bool {
    match hgt {
        Some(val) => {
            if val.len() < 2 {
                return false;
            }

            let unit = &val[val.len() - 2..];
            let height_str = &val[0..val.len() - 2];
            let height = match height_str.parse::<u32>() {
                Ok(x) => x,
                Err(_) => return false,
            };
            match unit {
                "cm" => return 150 <= height && height <= 193,
                "in" => return 59 <= height && height <= 76,
                _ => return false,
            };
        },
        None => return false,
    };
}

fn hcl_valid(hcl: Option<String>) -> bool {
    match hcl {
        Some(val) => {
            if val.len() != 7 ||
               val.chars().nth(0).unwrap() != '#' {
                return false;
            }

            match i32::from_str_radix(&val[1..], 16) {
                Ok(_) => return true,
                Err(_) => return false,
            };
        },
        None => return false,
    };
}

fn ecl_valid(ecl: Option<String>) -> bool {
    match ecl {
        Some(val) => {
            match "amb blu brn gry grn hzl oth".find(&val) {
                Some(_) => return true,
                None => return false,
            };
        }
        None => return false,
    };
}

fn pid_valid(pid: Option<String>) -> bool {
    match pid {
        Some(val) => {
            if val.len() != 9 {
                return false;
            }

            match val.parse::<u32>() {
                Ok(_) => return true,
                Err(_) => return false,
            };
        }
        None => return false,
    };
}
