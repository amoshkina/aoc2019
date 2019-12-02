use std::fs::read_to_string;
use std::error::Error;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(i32)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Op {
    Add = 1,
    Mult = 2,
    Halt = 99
}

fn restore_1202(codes: &mut Vec<i32>) {
    codes[1] = 12;
    codes[2] = 2;
}

fn get_args(codes: &Vec<i32>, current: usize) -> (i32, i32) {
    (codes[codes[current + 1 as usize] as usize], codes[codes[current + 2 as usize] as usize])
}

fn store_result(codes: &mut Vec<i32>, current: usize, result: i32) {
    let index = codes[current + 3 as usize] as usize;
    codes[index] = result
}

fn main() -> MyResult<()> {
    let mut codes: Vec<i32> = read_to_string("src/input.txt")?.split(',').map(|item| item.parse::<i32>().unwrap()).collect();
    // restore_1202(&mut codes);

    println!("codes: {:?}", codes);

    let mut op: Op = Op::Add;
    let mut current: usize = 0;
    
    while current < codes.len() {
        println!("current for op: {:?}", current);
        match Op::try_from(codes[current]) {
            Ok(Op::Add) => {
                println!("current = {:?}", current);
                let (arg1, arg2) = get_args(&codes, current);
                store_result(&mut codes, current, arg1 + arg2);
            },
            Ok(Op::Mult) => {
                println!("current = {:?}", current);
                let (arg1, arg2) = get_args(&codes, current);
                store_result(&mut codes, current, arg1 * arg2);
            },
            Ok(Op::Halt) => break,
            Err(_) => println!("Result Part 1: {:?}", codes[0])
        }

        current += 4;

    }
    println!("Finished without errors: {:?}", codes);
    Ok(())
}
