use std::fs::read_to_string;
use std::error::Error;


type MyResult<T> = Result<T, Box<dyn Error>>;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

fn phase(input: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for pos in 1..input.len()+1 {
        let pattern: Vec<i64> = [
            vec![BASE_PATTERN[0]; pos],
            vec![BASE_PATTERN[1]; pos],
            vec![BASE_PATTERN[2]; pos],
            vec![BASE_PATTERN[3]; pos],
        ].concat();
        let mut pattern_iter = pattern.iter().cycle();
        pattern_iter.next();
        let num: i64 = pattern_iter.zip(input.iter()).map(|(a, b)| a*b).sum::<i64>().abs() % 10;
        result.push(num);
    }

    result
}

fn main() -> MyResult<()> {
    let data: String = read_to_string("src/input.txt")?;
    let mut input: Vec<i64> = data.chars().map(|item| item.to_string().parse::<i64>().unwrap()).collect();

    for _ in 0..100 {
        input = phase(&input);
    }
    println!("Result Part 1: {:?}", input[0..8].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    
    Ok(())
}
