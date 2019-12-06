use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::error::Error;
use std::collections::{ HashMap};


type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug, Deserialize)]
struct Rel {
    center: String,
    satellite: String
}

fn construct_space() -> MyResult<HashMap<String, String>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut space: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = &line?;
        let rel: Rel = serde_scan::scan!("{}){}" <- line)?;
        space.insert(rel.satellite, rel.center);
    }

    Ok(space)
}

fn part1(space: HashMap<String, String>) -> u32 {
    let mut counter: u32 = 0;
    for (_satellite, center) in &space {
        let mut center: &String = center;
        counter += 1;
        while *center != "COM" {
            center = space.get(center).unwrap();
            counter += 1;
        }
    }
    counter
}

fn main() -> MyResult<()> {
    let space = construct_space()?;

    println!("Result Part 1: {:?}", part1(space));

    Ok(())
}
