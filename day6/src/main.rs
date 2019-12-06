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

fn main() -> MyResult<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut map: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = &line?;
        let rel: Rel = serde_scan::scan!("{}){}" <- line)?;
        map.insert(rel.satellite, rel.center);
    }

    let mut counter: u32 = 0;
    for (_satellite, center) in &map {
        let mut center: &String = center;
        counter += 1;
        while *center != "COM" {
            center = map.get(center).unwrap();
            counter += 1;
        }
    }

    println!("Result Part 1: {:?}", counter);

    Ok(())
}
