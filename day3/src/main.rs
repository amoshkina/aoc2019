use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;


type MyResult<T> = Result<T, Box<dyn Error>>;
type T = Vec<(i32, i32)>;

#[derive(Debug, Deserialize)]
enum Step {
    R(i32), 
    L(i32), 
    U(i32), 
    D(i32)
}


fn construct_wires() -> MyResult<Vec<T>> {
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
                        if (i, y) != (0, 0) {
                            wire.push((i, y));
                        }
                        
                    }
                    x += len;
                },
                Step::L(len) => {
                    for i in x-len..x {
                        if (i, y) != (0, 0) {
                            wire.push((i, y));
                        }
                    }
                    x -= len;
                },
                Step::U(len) => {
                    for j in y..y+len {
                        if (x, j) != (0, 0) {
                            wire.push((x, j));
                        }
                    }
                    y += len;
                },
                Step::D(len) => {
                    for j in y-len..y {
                        if (x, j) != (0, 0) {
                            wire.push((x, j));
                        }
                        
                    }
                    y -= len;
                }

            }
        }
        wires.push(wire);
    }
    Ok(wires)
}

fn intersections(wire1: &T, wire2: &T) -> HashMap<(i32, i32), i32> {
    let wire1: HashSet<&(i32, i32)> = HashSet::from_iter(wire1);
    let wire2: HashSet<&(i32, i32)> = HashSet::from_iter(wire2);
    wire1.intersection(&wire2).map(|&&coord| (coord, 0)).collect()
}

fn part1(crosses: &HashMap<(i32, i32), i32>) -> i32 {
    crosses.iter().map(|(coord, _)| coord.0.abs() + coord.1.abs()).min().unwrap()
}

fn part2(wires: &Vec<T>, crosses: &mut HashMap<(i32, i32), i32>) -> i32 {
    for wire in wires {
        let mut counter: i32 = 0;
        for point in wire {
            counter += 1;
            if let Some(count) = crosses.get_mut(point) {
                *count += counter;
            }
        }
    }

    *crosses.into_iter().min_by_key(|value| value.1.clone()).unwrap().1
}


fn main() -> MyResult<()> {
    let wires = construct_wires().unwrap();
    let mut crosses = intersections(&wires[0], &wires[1]);

    println!("Part 1 Result: {:?}", part1(&crosses));
    println!("Part 2 Result: {:?}", part2(&wires, &mut crosses));

    Ok(())
}
