use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryInto;

fn main() {
    let path = Path::new("input");

    let mut program = read_input(path);
    let noun_idx: usize = 1;
    let verb_idx: usize = 2;
    program[noun_idx] = 12;
    program[verb_idx] = 2;
    let output = run_program(&mut program);
    println!("output:{}", output);
}

fn run_program(program: &mut Vec<i32>) -> i32 {
    println!("{:?}", program);
    let mut pc: usize = 0;
    let mut opcode = program[pc];
    while opcode != 99 {
        let op1_idx: usize = program[pc+1].try_into().unwrap();
        let op1 = program[op1_idx];
        let op2_idx: usize = program[pc+2].try_into().unwrap();
        let op2 = program[op2_idx];
        let dest: usize = program[pc+3].try_into().unwrap();
        match opcode {
            1 => {
                //println!("Add {} {} dest:{}", op1, op2, dest);
                program[dest] = op1 + op2;
            },
            2 => {
                //println!("Multiply {} {} dest:{}", op1, op2, dest);
                program[dest] = op1 * op2;
            },
            _ => {
                println!("Unknown opcode:{}", opcode);
                break;
            },
        }
        pc += 4;
        opcode = program[pc];
    }

    return program[0];
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
    for num_str in s.split(',') {
        let num: i32 = match num_str.parse::<i32>() {
            Err(_) => continue,
            Ok(num) => num,
        };
        vec.push(num);
    }

    return vec;
}
