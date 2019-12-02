use std::fs::read_to_string;
use std::error::Error;

use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

use std::convert::TryFrom;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Op {
    Add = 1,
    Mult = 2,
    Halt = 99
}

#[derive(Debug)]
struct Intcode {
    code: Vec<usize>,
    ptr: usize
}

impl Intcode {
    fn new() -> Self {
        Self{
            code: read_to_string("src/input.txt").unwrap().split(',').map(|item| item.parse::<usize>().unwrap()).collect(),
            ptr: 0
        }
    }

    fn restore_1202(self: &mut Self) {
        self.code[1] = 12;
        self.code[2] = 2;
    }

    fn get_args(self: &Self) -> (usize, usize) {
        (self.code[self.code[self.ptr + 1]], self.code[self.code[self.ptr + 2]])
    }

    fn store_result(self: &mut Self, result: usize) {
        let index = self.code[self.ptr + 3];
        self.code[index] = result
    }

    fn finished(self: &Self) -> bool {
        self.ptr >= self.code.len()
    }

    fn op(self: &Self) -> Result<Op, TryFromPrimitiveError<Op>> {
        Op::try_from(self.code[self.ptr])
    }

    fn next(self: &mut Self) {
        self.ptr += 4
    }
}



fn main() -> MyResult<()> {
    let mut program = Intcode::new();
    program.restore_1202();

    while !program.finished() {
        match program.op()? {
            Op::Add => {
                let (arg1, arg2) = program.get_args();
                program.store_result(arg1 + arg2);
            },
            Op::Mult => {
                let (arg1, arg2) = program.get_args();
                program.store_result(arg1 * arg2);
            },
            Op::Halt => break,
        }

        program.next()

    }
    println!("Result Part 1: {:?}", program.code[0]);
    Ok(())
}
