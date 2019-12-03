use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use std::collections::HashSet;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
enum Step {
    R(i32), 
    L(i32), 
    U(i32), 
    D(i32)
}

type T = HashSet<(i32, i32)>;

fn main() -> MyResult<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut wires: Vec<T> = vec![];

    for line in reader.lines() {
        let mut wire: T = T::new();
        let (mut x, mut y): (i32, i32) = (0, 0);
        for value in line?.split(',').collect::<Vec<&str>>() {
            let step: Step = serde_scan::from_str(&format!("{} {}", &value[0..1], &value[1..]))?;

            // TODO: needs to shorten
            match step {
                Step::R(len) => {
                    for i in x..x+len {
                        wire.insert((i, y));
                    }
                    x += len;
                },
                Step::L(len) => {
                    for i in x-len..x {
                        wire.insert((i, y));
                    }
                    x -= len;
                },
                Step::U(len) => {
                    for j in y..y+len {
                        wire.insert((x, j));
                    }
                    y += len;
                },
                Step::D(len) => {
                    for j in y-len..y {
                        wire.insert((x, j));
                    }
                    y -= len;
                }

            }
        }
        wires.push(wire);
    }

    let intersections: HashSet<_> = wires[0].intersection(&wires[1]).collect();

    // TODO: fix &&& hell
    if let Some(nearest) = intersections.iter().filter(|&&&tuple| tuple != (0, 0)).map(|tuple| tuple.0.abs() + tuple.1.abs()).min() {
        println!("Part 1 Result: {:?}", nearest);
    }
    

    Ok(())
}
