use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryInto;

fn main() {
    let path = Path::new("input");

    let mut program = read_input(path);
    let mut io: Vec<i32> = vec![1];

    run_program(&mut program, &mut io);

    println!("output:");
    for v in io {
        println!("{}", v);
    }
}

enum OpMode {
    Pos,
    Imm,
}

struct Instr {
    modes: Vec<OpMode>,
    opcode: i32,
}

fn run_program(program: &mut Vec<i32>, io: &mut Vec<i32>) {
    //println!("{:?}", program);
    let mut pc: usize = 0;
    let mut instr: Instr = decode(program[pc]);
    while instr.opcode != 99 {
        //println!("pc:{}", pc);
        match instr.opcode {
            1 => {
                //println!("Add:{}", program[pc]);
                let (_, op1) = get_op(1, program, &instr, pc);
                let (_, op2) = get_op(2, program, &instr, pc);
                let (op3_value, _) = get_op(3, program, &instr, pc);
                //println!("Add {} {} dest:{}", op1, op2, dest);
                program[op3_value] = op1 + op2;
                pc += 4;
            },
            2 => {
                //println!("Multi:{}", program[pc]);
                let (_, op1) = get_op(1, program, &instr, pc);
                let (_, op2) = get_op(2, program, &instr, pc);
                let (op3_value, _) = get_op(3, program, &instr, pc);
                //println!("Multiply {} {} dest:{}", op1, op2, dest);
                program[op3_value] = op1 * op2;
                pc += 4;
            },
            3 => {
                //println!("Input:{}", program[pc]);
                let input = get_input(io);
                let (op1_value, _) = get_op(1, program, &instr, pc);
                program[op1_value] = input;
                pc += 2;

            },
            4 => {
                //println!("Output:{}", program[pc]);
                let (op1_value, _) = get_op(1, program, &instr, pc);
                write_output(program[op1_value], io);
                pc += 2;
            },
            _ => {
                panic!("Unknown opcode:{}", instr.opcode);
            },
        }
        instr = decode(program[pc]);
    }
}

fn decode(value: i32) -> Instr {
    let mut instruction = value;
    let mut modes: Vec<OpMode> = vec![OpMode::Pos, OpMode::Pos, OpMode::Pos];

    if instruction >= 10000 {
        modes[2] = OpMode::Imm;
        instruction -= 10000;
    }

    if instruction >= 1000 {
        modes[1] = OpMode::Imm;
        instruction -= 1000;
    }

    if instruction >= 100 {
        modes[0] = OpMode::Imm;
        instruction -= 100;
    }

    return Instr {
        modes: modes,
        opcode: instruction,
    };

}

fn get_input(io: &mut Vec<i32>) -> i32 {
        match io.pop() {
            Some(value) => return value,
            None => panic!("No inputs left"),
        }
}

fn write_output(value: i32, io: &mut Vec<i32>) {
    io.push(value);
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

fn get_op(op_num: usize, program: &mut Vec<i32>, instr: &Instr, pc: usize) -> (usize, i32) {
    return match instr.modes[op_num-1] {
        OpMode::Pos => {
            let op1_value: usize = program[pc+op_num].try_into().unwrap();
            (op1_value, program[op1_value])
        },
        OpMode::Imm => (0, program[pc+op_num]),
        _ => panic!("Unknown op mode"),
    };
}
