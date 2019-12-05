use std::io;
use std::fs::read_to_string;
use std::error::Error;
use std::io::stdout;
use std::io::Write;

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
            modes.push(acc % 10);
            acc = acc / 10;
        }

        let mut params: Vec<i32> = vec![];
        if instruction != 3 {
            for (&value, mode) in code[iptr+1..iptr+num+1].into_iter().zip(&modes) {
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
            4 => Op::Output(params[0] as usize),
            99 => Op::Halt,
            invalid => panic!("Invalid instruction code {:?}", invalid),
        }
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
                Op::Input(addr) => {
                    let mut input = String::new();
                    println!("");
                    print!("$ ");
                    stdout().flush();
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    let value: i32 = input.trim().parse().unwrap();
                    self.save(value, addr)
                },
                Op::Output(value) => {
                    println!("> {:?}", value);
                },
                Op::Halt => break,
            }

            self.next()

        }
        self.code[0]
    }
}

fn part1(data: &str) -> i32 {
    let mut program = Intcode::new(data);

    program.run()
}


fn main() -> MyResult<()> {
    let data: String = read_to_string("src/input.txt")?;
    part1(&data);
    
    Ok(())
}
