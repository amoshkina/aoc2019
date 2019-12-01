use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}


fn part1() -> MyResult<i32> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line in reader.lines() {
        let mass: i32 = line?.parse()?;
        total += fuel(mass)
    }

    Ok(total)
}

fn part2() -> MyResult<i32> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line in reader.lines() {
        let mut mass: i32 = line?.parse()?;
        let mut acc: i32 = 0;

        mass = fuel(mass);
        while mass > 0 {
            acc += mass;
            mass = fuel(mass);
        }

        total += acc;
    }
    Ok(total)
}


fn main() -> MyResult<()> {
    println!("Part 1 answer: {:?}", part1()?);
    println!("Part 2 answer: {:?}", part2()?);
    Ok(())
}
