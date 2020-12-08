use std::io;

use file_reader;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct PpData {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PpData {
    fn new() -> PpData {
        PpData {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

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
    if data.byr != None &&
       data.iyr != None &&
       data.eyr != None &&
       data.hgt != None &&
       data.hcl != None &&
       data.ecl != None &&
       data.pid != None {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "invalid pp"))
    }
}
