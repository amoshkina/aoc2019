use std::fs::read_to_string;
use std::error::Error;
use std::default::Default;
use std::collections::HashMap;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    Add(i64, i64, usize), // 1
    Mult(i64, i64, usize), // 2
    Input(usize), // 3
    Output(i64), // 4
    JumpTrue(i64, usize), // 5
    JumpFalse(i64, usize), // 6
    Less(i64, i64, usize), // 7
    Equals(i64, i64, usize), // 8
    AdjustBase(i64), // 9
    Halt, // 99
}

#[derive(Debug, Clone)]
struct Intcode {
    // TODO: refactor with autoresizable custom vector to avoid inlining "resize_with"
    code: Vec<i64>,
    iptr: usize,
    op: Op,
    base: i64
}


#[derive(Debug)]
struct IO {
    stream: Vec<i64>,
    blocking: bool
}

impl IO {
    fn new(blocking: bool) -> Self {
        Self{
            stream: vec![],
            blocking: blocking
        }
    }
}

fn resize(code: &mut Vec<i64>, addr: usize) {
    if addr >= code.len() {
        code.resize_with((addr+1) as usize, Default::default)
    }
}


impl Intcode {
    fn new(data: &str) -> Self {
        // FIXME: unwrap
        let mut code: Vec<i64> = data.split(',').map(|item| item.parse::<i64>().unwrap()).collect();
        let iptr: usize = 0;
        let base: i64 = 0;
        let op = Self::parse_op(&mut code, iptr, base);
        Self{code, iptr, op, base}
    }

    fn parse_op(code: &mut Vec<i64>, iptr: usize, base: i64) -> Op {
        let instruction = code[iptr] % 100; 
        let mut acc = code[iptr] / 100;
        let mut modes: Vec<i64> = vec![];
        let num: usize = match instruction {
            1 | 2 | 7 | 8 => 3,
            3 | 4 => 1,
            5 | 6 => 2,
            9     => 1,
            99    => 0,
            invalid => panic!("Invalid instruction code {:?}", invalid),
        };

        for _ in 0..num {
            modes.push(acc % 10);
            acc = acc / 10;
        }

        let mut params: Vec<i64> = vec![];
        let values: Vec<i64> = code[iptr+1..iptr+num+1].iter().cloned().collect();
        for (&value, mode) in values.iter().zip(&modes) {
            let param: i64 = match mode {
                0 => {
                    resize(code, value as usize);
                    code[value as usize]
                },
                1 => value,
                2 => {
                    resize(code, (base+value) as usize);
                    code[(base + value) as usize]
                },
                invalid => panic!("Invalid mode identifier: {:?}", invalid)
            };
            params.push(param);
        }
        
        // FIXME: this is some sort of a hack, I do not like it, but normal flow with "params" doesn't work
        let mut write_addr: usize = code[iptr+num] as usize;
        if modes.len() > 0 && modes[modes.len()-1] == 2 {
            write_addr = (code[iptr+num] + base) as usize;
        }

        match instruction {
            1 => Op::Add(params[0], params[1], write_addr),
            2 => Op::Mult(params[0], params[1], write_addr),
            3 => Op::Input(write_addr),
            4 => Op::Output(params[0]),
            5 => Op::JumpTrue(params[0], params[1] as usize),
            6 => Op::JumpFalse(params[0], params[1] as usize),
            7 => Op::Less(params[0], params[1], write_addr),
            8 => Op::Equals(params[0], params[1], write_addr),
            9 => Op::AdjustBase(params[0]),
            99 => Op::Halt,
            invalid => panic!("Invalid instruction code {:?}", invalid),
        }
    }

    fn save(self: &mut Self, result: i64, addr: usize) {
        resize(&mut self.code, addr);
        self.code[addr] = result;
    }

    fn finished(self: &Self) -> bool {
        self.iptr >= self.code.len()
    }

    fn params_num(self: &Self) -> usize {
        match self.op {
            Op::Add(_, _ ,_) | Op::Mult(_, _, _) | Op::Less(_, _, _) | Op::Equals(_, _, _) => 3,
            Op::JumpTrue(_, _) | Op::JumpFalse(_, _) => 2,
            Op::Input(_) | Op::Output(_) | Op::AdjustBase(_) => 1,
            Op::Halt => 0
        }    
    }

    fn next(self: &mut Self, addr: Option<usize>) {
        let addr = match addr {
            Some(value) => value,
            None => self.iptr + self.params_num() + 1, // adding 1 as op itself takes one place in code along with params
        };

        self.iptr = addr;
        self.op = Self::parse_op(&mut self.code, self.iptr, self.base);
    }

    fn run(self: &mut Self, input: &mut IO, output: &mut IO) -> i64 {
        while !self.finished() {
            let mut next_addr: Option<usize> = None;
            match self.op {
                Op::Add(value1, value2, addr) => self.save(value1 + value2, addr),

                Op::Mult(value1, value2, addr) => self.save(value1 * value2, addr),

                Op::Input(addr) => {
                    let value: i64 = input.stream.pop().unwrap();
                    self.save(value, addr);
                    if input.blocking {
                        self.next(next_addr);
                        return 3
                    }
                    
                },

                Op::Output(value) => {
                    output.stream.push(value);
                    if output.blocking {
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

                Op::AdjustBase(value) => self.base += value,

                Op::Halt => break,
            }
            self.next(next_addr)
        }
        0
    }
}

// fn part(data: &str, param: i64) -> i64 {
//     let (mut input, mut output): (IO, IO);  

//     let mut program = Intcode::new(&data);
//     input = IO::new(false);
//     output = IO::new(false);
//     input.stream.push(param);
//     program.run(&mut input, &mut output);
//     // FIXME: unwrap
//     output.stream.pop().unwrap()
// }

fn step((x, y): (i64, i64), dir: &Dir) -> (i64, i64) {
    match &dir {
        Dir::Up    => (x, y+1),
        Dir::Right => (x+1, y),
        Dir::Left  => (x-1, y), 
        Dir::Down  => (x, y-1)
    }
}

fn turn(dir: Dir, to: i64) -> Dir {
    match to {
        LEFT_90 => match dir {
            Dir::Up    => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down  => Dir::Right,
            Dir::Left  => Dir::Down, 
           
        },
        RIGHT_90 => match dir {
            Dir::Up    => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down  => Dir::Left,
            Dir::Left  => Dir::Up, 
           
        },
        invalid => panic!("Invalid turn option: {}", invalid)
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

const BLACK: i64 = 0;
const LEFT_90: i64 = 0;
const RIGHT_90: i64 = 1;


fn main() -> MyResult<()> {
    let data: String = read_to_string("src/input.txt")?;

    let (mut input, mut output): (IO, IO);  
    input = IO::new(false);
    output = IO::new(true);

    let mut brain = Intcode::new(&data);
    let mut panels: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos: (i64, i64) = (0, 0);
    let mut dir: Dir = Dir::Up;
    loop {
        let color = panels.entry(pos).or_insert(BLACK);
        input.stream.push(*color);
        if brain.run(&mut input, &mut output) == 0 {
            break;
        }
        let new_color = output.stream.pop().unwrap();
        if brain.run(&mut input, &mut output) == 0 {
            break;
        }
        let turn_to = output.stream.pop().unwrap();
        *color = new_color;
        dir = turn(dir, turn_to);
        pos = step(pos, &dir);
    }
    // let mut keys = panels.keys().collect::<Vec<&(i64, i64)>>();
    // keys.sort();
    // println!("{:?}", keys);
    // for (panel, _) in panels.iter() {
    //     println!("{:?}", panel);
    // }
    
    println!("Result Part 1: {:?}", panels.len());
    
    Ok(())
}
