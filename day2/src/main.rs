use std::fs::read_to_string;
use std::error::Error;

use num_enum::TryFromPrimitive;
// use num_enum::Err;

use std::convert::TryFrom;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(i32)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Op {
    Add = 1,
    Mult = 2,
    Halt = 99
}

#[derive(Debug)]
struct Intcode {
    code: Vec<i32>,
    current: usize
}

impl Intcode {
    fn restore_1202(self: &mut Self) {
        self.code[1] = 12;
        self.code[2] = 2;
    }

    fn get_args(self: &Self) -> (i32, i32) {
        (self.code[self.code[self.current + 1 as usize] as usize], self.code[self.code[self.current + 2 as usize] as usize])
    }

    fn store_result(self: &mut Self, result: i32) {
        let index = self.code[self.current + 3 as usize] as usize;
        self.code[index] = result
    }

    fn finished(self: &Self) -> bool {
        self.current >= self.code.len()
    }

    fn op(self: &Self) -> Result<Op, num_enum::TryFromPrimitiveError<Op>> {
        Op::try_from(self.code[self.current])
    }

    fn next(self: &mut Self) {
        self.current += 4
    }
}



fn main() -> MyResult<()> {
    let mut program: Intcode = Intcode{
        code: read_to_string("src/input.txt")?.split(',').map(|item| item.parse::<i32>().unwrap()).collect(),
        current: 0
    };
    // program.restore_1202(&mut program);

    println!("program: {:?}", program);

    
    while !program.finished() {
        match program.op() {
            Ok(Op::Add) => {
                let (arg1, arg2) = program.get_args();
                program.store_result(arg1 + arg2);
            },
            Ok(Op::Mult) => {
                let (arg1, arg2) = program.get_args();
                program.store_result(arg1 * arg2);
            },
            Ok(Op::Halt) => break,
            Err(_) => println!("Result Part 1: {:?}", program.code[0])
        }

        program.next()

    }
    println!("Finished without errors: {:?}", program);
    Ok(())
}
