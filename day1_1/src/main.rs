use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input");

    let inputs = read_input(path);
    let mut output: i32 = 0;
    for num in inputs {
        let output_num = (num / 3) - 2;
        output += output_num;
    }

    println!("Total fuel requirement is:{}", output);
}

fn read_input(path: &Path) -> Vec<i32> {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read input");

    let mut vec: Vec<i32> = Vec::new();
    for num_str in s.lines() {
        let num: i32 = match num_str.parse::<i32>() {
            Err(why) => panic!("Couldn't parse string to number: {}", why),
            Ok(num) => num,
        };
        vec.push(num);
    }

    return vec;
}
