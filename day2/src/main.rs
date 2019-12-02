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

#[derive(Debug)]
struct Codes {
    value: Vec<i32>,
    current: usize
}

impl Codes {
    fn restore_1202(self: &mut Self) {
        self.value[1] = 12;
        self.value[2] = 2;
    }

    fn get_args(self: &Self) -> (i32, i32) {
        (self.value[self.value[self.current + 1 as usize] as usize], self.value[self.value[self.current + 2 as usize] as usize])
    }

    fn store_result(self: &mut Self, result: i32) {
        let index = self.value[self.current + 3 as usize] as usize;
        self.value[index] = result
    }
}



fn main() -> MyResult<()> {
    let value: Vec<i32> = read_to_string("src/input.txt")?.split(',').map(|item| item.parse::<i32>().unwrap()).collect();
    let mut codes: Codes = Codes{
        value: value,
        current: 0
    };
    // restore_1202(&mut codes);

    println!("codes: {:?}", codes);

    
    while codes.current < codes.value.len() {
        println!("current for op: {:?}", codes.current);
        match Op::try_from(codes.value[codes.current]) {
            Ok(Op::Add) => {
                println!("current = {:?}", codes.current);
                let (arg1, arg2) = codes.get_args();
                codes.store_result(arg1 + arg2);
            },
            Ok(Op::Mult) => {
                println!("current = {:?}", codes.current);
                let (arg1, arg2) = codes.get_args();
                codes.store_result(arg1 * arg2);
            },
            Ok(Op::Halt) => break,
            Err(_) => println!("Result Part 1: {:?}", codes.value[0])
        }

        codes.current += 4;

    }
    println!("Finished without errors: {:?}", codes);
    Ok(())
}
