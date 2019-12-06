use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;


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

fn construct_path<'a>(space: &'a HashMap<String, String>, from: &'a str, to: &str) -> HashMap<&'a str, u32> {
    let mut from = from;

    let mut path: HashMap<&str, u32> = HashMap::new();
    let mut counter: u32 = 0;
    
    while from != to {
        from = space.get(from).unwrap();
        path.insert(&from, counter);
        counter += 1;
    }
    path
} 

fn part1(space: &HashMap<String, String>) -> u32 {
    let mut counter: u32 = 0;
    for (_satellite, center) in space {
        let mut center: &String = center;
        counter += 1;
        while *center != "COM" {
            center = space.get(center).unwrap();
            counter += 1;
        }
    }
    counter
}

fn part2(space: &HashMap<String, String>) -> u32 {
    let you_com = construct_path(&space, "YOU", "COM");
    let san_com = construct_path(&space, "SAN", "COM");

    let you: HashSet<&str> = HashSet::from_iter(you_com.iter().map(|(object, _)| *object));
    let san: HashSet<&str> = HashSet::from_iter(san_com.iter().map(|(object, _)| *object));

    let common: HashSet<_> = you.intersection(&san).collect();

    let min_you = you_com.iter().filter(|(object, _)| common.contains(object)).map(|(_, dist)| dist).min();
    let min_san = san_com.iter().filter(|(object, _)| common.contains(object)).map(|(_, dist)| dist).min();
    min_you.unwrap() + min_san.unwrap()
}

fn main() -> MyResult<()> {
    let space = construct_space()?;

    println!("Result Part 1: {:?}", part1(&space));
    println!("Result Part 2: {:?}", part2(&space));

    Ok(())
}
