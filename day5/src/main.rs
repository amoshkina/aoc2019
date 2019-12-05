use std::fs::read_to_string;
use std::error::Error;
use std::convert::TryFrom;

use num_enum::{TryFromPrimitive};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Op {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    Halt = 99
}

#[derive(Debug)]
struct Intcode {
    code: Vec<usize>,
    iptr: usize,
    op: Op
}

impl Intcode {
    fn new(data: &str) -> Self {
        let code = data.split(',').map(|item| item.parse::<usize>().unwrap()).collect();
        let iptr: usize = 0;
        let op = Self::parse_op(&code, iptr);
        Self{code, iptr, op}
    }

    fn load_input(self: &mut Self, param1: usize, param2: usize) {
        self.code[1] = param1;
        self.code[2] = param2;
    }

    // TODO: get_args and store_result are no longer convenient
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

    fn parse_op(code: &Vec<usize>, iptr: usize) -> Op {
        Op::try_from(code[iptr]).unwrap()
    }

    fn params_num(self: &Self) -> usize {
        match self.op {
            Op::Add | Op::Mult => 3,
            Op::Input | Op::Output => 1,
            Op::Halt => 0
        }    
    }

    fn next(self: &mut Self) {
        // adding 1 as op itself takes one place in code along with params
        self.iptr += self.params_num() + 1;
        self.op = Self::parse_op(&self.code, self.iptr)
    }

    fn run(self: &mut Self) -> usize {
        while !self.finished() {
            match self.op {
                Op::Add => {
                    let (arg1, arg2) = self.get_args();
                    self.store_result(arg1 + arg2);
                },
                Op::Mult => {
                    let (arg1, arg2) = self.get_args();
                    self.store_result(arg1 * arg2);
                },
                Op::Halt => break,
                Op::Input => unimplemented!(),
                Op:: Output => unimplemented!()
            }

            self.next()

        }
        self.code[0]
    }
}

fn part1(data: &str) -> usize {
    let mut program = Intcode::new(data);
    program.load_input(12, 2);

    program.run()
}

fn part2(data: &str) -> usize {
    const EXPECTED: usize = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = Intcode::new(data);
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
    let data: String = read_to_string("src/input.txt")?;
    println!("Result Part 1: {:?}", part1(&data));
    println!("Result Part 2: {:?}", part2(&data));
    Ok(())
}
