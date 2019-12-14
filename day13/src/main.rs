use std::fs::read_to_string;
use std::error::Error;
use std::default::Default;


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
    fn new(data: &str, ) -> Self {
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
                    assert!(!input.stream.is_empty());
                    let value: i64 = input.stream.pop().unwrap();
                    println!("read input: {:?}", value);
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

fn part1(data: &str) -> usize {
    let mut input = IO::new(false);
    let mut output = IO::new(false);

    let mut program = Intcode::new(&data);
    program.run(&mut input, &mut output);
    output.stream.iter().enumerate().filter(|(i, item)| (i + 1) % 3 == 0 && **item == 2).count()
}

#[derive(Debug)]
struct Layout {
    grid: Vec<Vec<char>>,
    ball: (i64, i64),
    ball_x_rate: i64,
    ball_y_rate: i64,
    paddle: (i64, i64),
    score: i64,
    blocks: i64
}

impl Layout {
    fn new() -> Self {
        Self{
            grid: vec![vec![' '; 42]; 23],
            ball: (-1, -1),
            ball_x_rate: 1,
            ball_y_rate: 1,
            paddle: (-1, -1),
            score: 0,
            blocks: 0
        }
    }

    fn update_layout(self: &mut Self, stream: &mut Vec<i64>) {
        while !stream.is_empty() {
            let key = stream.pop().unwrap();
            let y = stream.pop().unwrap();
            let x = stream.pop().unwrap();
            if x == -1 && y == 0 {
                // a hit occurred, a ball changed it's direction
                self.score = key;
            } else {

                let tile = match key {
                    0 => {
                        if self.grid[y as usize][x as usize] == 'Z' {
                            // a block was broken, decreasing counter
                            self.blocks -= 1;

                            // self.ball_x_rate *= -1;
                            // self.ball_y_rate *= -1;
                            // println!("Broken a block!");
                            // println!("Changing x-rate to {:?}", self.ball_x_rate);
                            // println!("Changing y-rate to {:?}", self.ball_y_rate);
                        }
                        ' '
                    },
                    1 => '|',
                    2 => {
                        self.blocks += 1;
                        'Z'
                    },
                    3 => {
                        self.paddle = (x, y);
                        '_'
                    },
                    4 => {
                        self.ball = (x, y);
                        'O'
                    },
                    invalid => panic!("Invalid key: {:?}", invalid)
                };

                self.grid[y as usize][x as usize] = tile;
            }
        }
    }
}

fn print_layout(layout: &Layout) -> MyResult<()> {
    for (i, line) in layout.grid.iter().enumerate() {
        let mut line: String = line.iter().map(|&ch| ch.to_string()).collect();
        if i == 0 {
            line.push(' ');
            line.push_str(&layout.score.to_string());
        }
        println!("{}", line);
    }
    Ok(())
}

fn calculate_shift(layout: &Layout) -> i64 {
    let mut ball = layout.ball;
    let mut x_rate = layout.ball_x_rate;
    let mut y_rate = layout.ball_y_rate;

    println!("ball initial: {:?}", ball);
    println!("paddle current: {:?}", layout.paddle);
    println!("x_rate: {:?}, y_rate: {:?}", x_rate, y_rate);


    // making the first step outside the loop to cover the case when ball has just bumped off paddle
    ball = (ball.0 + x_rate, ball.1 + y_rate);

    while ball.1 + 1 != layout.paddle.1 {
        println!("ball wip: {:?}", ball);
        ball = (ball.0 + x_rate, ball.1 + y_rate);
        println!("rates wip= x: {:?}, y: {:?}", x_rate, y_rate);
        // TODO: walls also change direction!!!
        if layout.grid[ball.1 as usize][ball.0 as usize] == 'Z' {
            y_rate *= -1;
            println!("Simulated bump over block, changing y direction to {:?}", y_rate);
        }
    }

    let shift: i64;
    if ball.0 == layout.paddle.0 {
        shift = 0;
    } else if ball.0 > layout.paddle.0 {
        shift = 1;
    } else {
        shift = -1
    }
    println!("calculated shift: {:?}", shift);
    shift
}

fn part2(data: &str) -> MyResult<()> {
    let mut input = IO::new(true);
    let mut output = IO::new(false);

    let mut program = Intcode::new(&data);
    let mut layout: Layout = Layout::new();

    input.stream.push(0);

    let mut step: i64 = 0;
    while input.stream.len() > 0 {
        program.run(&mut input, &mut output);
        layout.update_layout(&mut output.stream);

        println!("---------------------------------------------------");
        print_layout(&layout);
        // TODO: walls also change direction!!!
        if layout.ball == (layout.paddle.0, layout.paddle.1 - 1) {
            // bump over paddle has occurred, changing ball directions
            layout.ball_y_rate *= -1;
            println!("Bump over paddle!");
            println!("Changing y-rate to {:?}", layout.ball_y_rate);

        } else if layout.grid[layout.ball.1 as usize -1][layout.ball.0 as usize] == 'Z' {
            // TODO: probably ball can break a block when one tile to right or left
            layout.ball_y_rate *= -1;
            println!("Broken a block!");
            println!("Changing y-rate to {:?}", layout.ball_y_rate);
        }

        let joystick = calculate_shift(&layout);
        input.stream.push(joystick);
        step += 1;
    }
    Ok(())
}

fn main() -> MyResult<()> {
    let data: String = read_to_string("src/input.txt")?;

    // println!("Result Part 1: {:?}", part1(&data));
    part2(&data)?;
    Ok(())
}




        // println!("input len: {:?}", input.stream.len());
        // println!("step: {:?}", step);
        // println!("stream: {:?}", output.stream);
        
        // println!("ball position: {:?}", layout.ball);
        
        // println!("state: {:?}, iptr: {:?}", program.code[program.iptr], program.iptr);