use std::fs::read_to_string;
use std::error::Error;
use std::mem::swap;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Range;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    Add(i32, i32, usize), // 1
    Mult(i32, i32, usize), // 2
    Input(usize), // 3
    Output(i32), // 4
    JumpTrue(i32, usize), // 5
    JumpFalse(i32, usize), // 6
    Less(i32, i32, usize), // 7
    Equals(i32, i32, usize), // 8
    Halt, // 99
}

#[derive(Debug, Clone)]
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
            1 | 2 | 7 | 8 => 3,
            3 | 4 => 1,
            5 | 6 => 2,
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
            4 => Op::Output(params[0]),
            5 => Op::JumpTrue(params[0], params[1] as usize),
            6 => Op::JumpFalse(params[0], params[1] as usize),
            7 => Op::Less(params[0], params[1], code[iptr+num] as usize),
            8 => Op::Equals(params[0], params[1], code[iptr+num] as usize),
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
            Op::Add(_, _ ,_) | Op::Mult(_, _, _) | Op::Less(_, _, _) | Op::Equals(_, _, _) => 3,
            Op::JumpTrue(_, _) | Op::JumpFalse(_, _) => 2,
            Op::Input(_) | Op::Output(_) => 1,
            Op::Halt => 0
        }    
    }

    fn next(self: &mut Self, addr: Option<usize>) {
        let addr = match addr {
            Some(value) => value,
            None => self.iptr + self.params_num() + 1, // adding 1 as op itself takes one place in code along with params
        };

        self.iptr = addr;
        self.op = Self::parse_op(&self.code, self.iptr);
    }

    fn run(self: &mut Self, input: &mut Vec<i32>, output: &mut Vec<i32>, input_block: bool, output_block: bool) -> i32 {
        while !self.finished() {
            let mut next_addr: Option<usize> = None;
            match self.op {
                Op::Add(value1, value2, addr) => self.save(value1 + value2, addr),

                Op::Mult(value1, value2, addr) => self.save(value1 * value2, addr),

                Op::Input(addr) => {
                    let value: i32 = input.pop().unwrap();
                    self.save(value, addr);
                    if input_block {
                        self.next(next_addr);
                        return 3
                    }
                    
                },

                Op::Output(value) => {
                    output.push(value);
                    if output_block {
                        self.next(next_addr);
                        return 4
                    }
                },

                Op::JumpTrue(value, addr) => if value != 0 { next_addr = Some(addr) }, 

                Op::JumpFalse(value, addr) => if value == 0 { next_addr = Some(addr) }, 

                Op::Less(value1, value2, addr) => {
                    let result = if value1 < value2 { 1 } else { 0 };
                    self.code[addr] = result;
                },

                Op::Equals(value1, value2, addr) => {
                    let result = if value1 == value2 { 1 } else { 0 };
                    self.code[addr] = result;
                },

                Op::Halt => break,
            }
            self.next(next_addr)
        }
        0
    }
}


fn permutate(set: HashSet<i32>) -> Vec<Vec<i32>> {
    if set.is_empty() {
        return vec![vec![]]
    }
    let mut result: Vec<Vec<i32>> = vec![];
    for &item in &set {
        let mut other = set.clone();
        other.remove(&item);
        let smaller = permutate(other);
        for mut s in smaller {
            s.push(item);
            result.push(s);
        }
    }
    result
}

fn part1(data: &str) -> MyResult<i32> {
    let (mut input, mut output): (Vec<i32>, Vec<i32>);

    let mut result: i32 = 0;

    let set = init_set(0..5);

    for phases in permutate(set) {
        input = vec![];
        output = vec![0];
        for &phase in &phases {
            swap(&mut input, &mut output);
            input.push(phase);

            let mut program = Intcode::new(&data);
            program.run(&mut input, &mut output, false, false);           
        }
        result = max(result, output.pop().unwrap());
    }
    Ok(result)
}

fn init_set(range: Range<i32>) -> HashSet<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for item in range { set.insert(item); }
    set
}

fn feedback_loop(amplifiers: &mut Vec<Intcode>) -> i32 {
    let (mut input, mut output): (Vec<i32>, Vec<i32>);
    let mut i: usize = 0;
    input = vec![0];
    output = vec![];    
    loop {
        assert!(output.is_empty());
        assert!(!input.is_empty());
        
        let ret = amplifiers[i].run(&mut input, &mut output, false, true);
        swap(&mut input, &mut output);
        if ret == 0 {
            break;
        }
        
        i = (i + 1) % 5;
    }
    output.pop().unwrap()
}


fn part2(data: &str) -> MyResult<i32> {
    let (mut input, mut output): (Vec<i32>, Vec<i32>);
    let mut amplifiers: Vec<Intcode>;
    let mut result: i32 = 0;
    let set = init_set(5..10);


    for phases in permutate(set) {
        input = vec![];
        output = vec![];
        amplifiers = vec![Intcode::new(&data); 5];
        // initializing amplifiers with the phase
        for (i, &phase) in phases.iter().enumerate() {
            input.push(phase);
            amplifiers[i].run(&mut input, &mut output, true, true);
            assert!(input.is_empty());
        }
        result = max(result, feedback_loop(&mut amplifiers));
    }
    Ok(result)
}


fn main() -> MyResult<()> {
    let data: String = read_to_string("src/input.txt")?;

    println!("Result Part 1: {:?}", part1(&data)?);
    println!("Result Part 2: {:?}", part2(&data)?);
    
    Ok(())
}
