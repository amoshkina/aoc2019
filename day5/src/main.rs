use std::fs::read_to_string;
use std::error::Error;

use num_enum::{TryFromPrimitive};

use std::ops::Range;
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
    iptr: usize
}

impl Intcode {
    fn new() -> Self {
        Self{
            code: read_to_string("src/input.txt").unwrap().split(',').map(|item| item.parse::<usize>().unwrap()).collect(),
            iptr: 0
        }
    }

    fn load_input(self: &mut Self, param1: usize, param2: usize) {
        self.code[1] = param1;
        self.code[2] = param2;
    }

    fn get_args(self: &Self) -> (usize, usize) {
        (self.code[self.code[self.iptr + 1]], self.code[self.code[self.iptr + 2]])
    }

    fn store_result(self: &mut Self, result: usize) {
        let index = self.code[self.iptr + 3];
        self.code[index] = result
    }

    fn finished(self: &Self) -> bool {
        self.iptr >= self.code.len()
    }

    fn op(self: &Self) -> Op {
        Op::try_from(self.code[self.iptr]).unwrap()
    }

    fn next(self: &mut Self) {
        // TODO: needs to parametrise depending on current op
        self.iptr += 4
    }

    fn run(self: &mut Self) -> usize {
        while !self.finished() {
            match self.op() {
                Op::Add => {
                    let (arg1, arg2) = self.get_args();
                    self.store_result(arg1 + arg2);
                },
                Op::Mult => {
                    let (arg1, arg2) = self.get_args();
                    self.store_result(arg1 * arg2);
                },
                Op::Halt => break,
            }

            self.next()

        }
        self.code[0]
    }
}

fn part1() -> usize {
    let mut program = Intcode::new();
    program.load_input(12, 2);

    program.run()
}

fn part2() -> usize {
    const EXPECTED: usize = 19690720;
    for noun in (Range{start: 0, end: 100}) {
        for verb in (Range{start:0, end: 100}) {
            let mut program = Intcode::new();
            program.load_input(noun, verb);

            let result = program.run();
            if result == EXPECTED {
                return 100 * noun + verb
            }
        }
    }

    panic!("Correct pair of noun/verb is not found for {}", EXPECTED);
}


fn main() -> MyResult<()> {
    println!("Result Part 1: {:?}", part1());
    println!("Result Part 2: {:?}", part2());
    Ok(())
}
