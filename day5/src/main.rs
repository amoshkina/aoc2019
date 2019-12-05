use std::fs::read_to_string;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add(i32, i32, usize), // 1
    Mult(i32, i32, usize), // 2
    Input(usize), // 3
    Output(usize), // 4
    Halt, // 99
}

#[derive(Debug)]
struct Intcode {
    code: Vec<i32>,
    iptr: usize,
    op: Op
}

impl Intcode {
    fn new(data: &str) -> Self {
        // FIXME: unwrap
        let code: Vec<i32> = data.split(',').map(|item| item.parse::<i32>().unwrap()).collect();
        let iptr: usize = 0;
        let op = Self::parse_op(&code, iptr);
        Self{code, iptr, op}
    }

    fn parse_op(code: &Vec<i32>, iptr: usize) -> Op {
        let instruction = code[iptr] % 100; 
        let mut acc = code[iptr] / 100;
        let mut modes: Vec<i32> = vec![];
        let num: usize = match instruction {
            1 | 2 => 3,
            3 | 4 => 1,
            99    => 0,
            invalid => panic!("Invalid instruction code {:?}", invalid),
        };

        for _ in 0..num {
            modes.push(acc % 100);
            acc = acc / 100;
        }
        modes.reverse();

        let mut params: Vec<i32> = vec![];
        if num > 1 {
            for (&value, mode) in code[iptr+1..iptr+num].into_iter().zip(&modes) {
                let param: i32 = match mode {
                    0 => code[value as usize],
                    1 => value,
                    invalid => panic!("Invalid mode identifier: {:?}", invalid)
                };
                params.push(param);
            }
        }

        match instruction {
            1 => Op::Add(params[0], params[1], code[iptr+num] as usize),
            2 => Op::Mult(params[0], params[1], code[iptr+num] as usize),
            3 => Op::Input(code[iptr+num] as usize),
            4 => Op::Output(code[iptr+num] as usize),
            99 => Op::Halt,
            invalid => panic!("Invalid instruction code {:?}", invalid),
        }
    }

    fn load_input(self: &mut Self, param1: i32, param2: i32) {
        self.code[1] = param1;
        self.code[2] = param2;
    }

    fn save(self: &mut Self, result: i32, addr: usize) {
        self.code[addr] = result;
    }

    fn finished(self: &Self) -> bool {
        self.iptr >= self.code.len()
    }

    fn params_num(self: &Self) -> usize {
        match self.op {
            Op::Add(_, _ ,_) | Op::Mult(_, _, _) => 3,
            Op::Input(_) | Op::Output(_) => 1,
            Op::Halt => 0
        }    
    }

    fn next(self: &mut Self) {
        // adding 1 as op itself takes one place in code along with params
        self.iptr += self.params_num() + 1;
        self.op = Self::parse_op(&self.code, self.iptr)
    }

    fn run(self: &mut Self) -> i32 {
        while !self.finished() {
            match self.op {
                Op::Add(value1, value2, addr) => self.save(value1 + value2, addr),
                Op::Mult(value1, value2, addr) => self.save(value1 * value2, addr),
                Op::Input(_addr) => unimplemented!(),
                Op::Output(_addr) => unimplemented!(),
                Op::Halt => break,
            }

            self.next()

        }
        self.code[0]
    }
}

fn part1(data: &str) -> i32 {
    let mut program = Intcode::new(data);
    program.load_input(12, 2);

    program.run()
}

fn part2(data: &str) -> i32 {
    const EXPECTED: i32 = 19690720;
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
