use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::iter::Iterator;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
enum Step {
    R(i32), 
    L(i32), 
    U(i32), 
    D(i32)
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn construct_wires() -> MyResult<Vec<Vec<(Point)>>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut wires: Vec<Vec<Point>> = vec![];

    for line in reader.lines() {
        let mut wire: Vec<Point> = vec![];
        let (mut x, mut y): (i32, i32) = (0, 0);
        for value in line?.split(',') {
            let step: Step = serde_scan::from_str(&format!("{} {}", &value[0..1], &value[1..]))?;

            let xs: Vec<i32>;
            let ys: Vec<i32>;

            match step {
                Step::R(len) => {
                    xs = (x..x+len).collect();
                    ys = vec![y; len as usize];
                    x += len;
                },
                Step::L(len) => {
                    xs = (x-len+1..x+1).rev().collect();
                    ys = vec![y; len as usize];
                    x -= len;
                },
                Step::U(len) => {
                    xs = vec![x; len as usize];
                    ys = (y..y+len).collect();
                    y += len;
                },
                Step::D(len) => {
                    xs = vec![x; len as usize];
                    ys = (y-len+1..y+1).rev().collect();
                    y -= len;
                }

            }
            for (&x, &y) in xs.iter().zip(&ys) {
                if (x, y) != (0, 0) {
                    wire.push(Point{x, y})
                }
            }
            
        }
        wires.push(wire);
    }
    Ok(wires)
}

fn intersections(wire1: &Vec<Point>, wire2: &Vec<Point>) -> HashMap<Point, i32> {
    let wire1: HashSet<&Point> = HashSet::from_iter(wire1);
    let wire2: HashSet<&Point> = HashSet::from_iter(wire2);
    wire1.intersection(&wire2).map(|&&point| (point, 0)).collect()
}

fn part1(crosses: &HashMap<Point, i32>) -> MyResult<i32> {
    crosses.keys().map(|point| point.x.abs() + point.y.abs()).min().ok_or(Box::from("error"))
}

fn part2(wires: Vec<Vec<Point>>, crosses: &mut HashMap<Point, i32>) -> MyResult<&i32> {
    for wire in wires {
        let mut counter: i32 = 0;
        for point in wire {
            counter += 1;
            if let Some(count) = crosses.get_mut(&point) {
                *count += counter;
            }
        }
    }

    crosses.values().min().ok_or(Box::from("error"))
}


fn main() -> MyResult<()> {
    let wires = construct_wires().unwrap();
    let mut crosses = intersections(&wires[0], &wires[1]);

    println!("Part 1 Result: {:?}", part1(&crosses)?);
    println!("Part 2 Result: {:?}", part2(wires, &mut crosses)?);

    Ok(())
}
