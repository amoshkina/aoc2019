use serde_scan;
use serde::Deserialize;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32
}


#[derive(Debug, PartialEq, Copy, Clone)]
struct Moon {
    pos: Position,
    vel: Velocity
}

impl Moon {
    fn new(pos: Position, vel: Velocity) -> Self {
        Self{pos: pos, vel: vel}
    }

    fn e_potential(self: &Self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn e_kinetic(self: &Self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn e_total(self: &Self) -> i32 {
        self.e_potential() * self.e_kinetic()
    }
}

fn apply_gravity(pos_i1: &mut i32, vel_i1: &mut i32, pos_i2: &mut i32, vel_i2: &mut i32) {
    if *pos_i1 > *pos_i2 {
        *vel_i1 -= 1;
        *vel_i2 += 1;
    } else if *pos_i1 < *pos_i2 {
        *vel_i1 += 1;
        *vel_i2 -= 1;
    }
}

fn debug_print(moons: &Vec<Moon>, current: i32) {
    println!("--------------------------");
    println!("step: {:?}", current);
    for moon in moons {
        println!(
            "pos=<x={:?}, y=  {:?}, z= {:?}>, vel=<x= {:?}, y= {:?}, z= {:?}>", 
            moon.pos.x, moon.pos.y, moon.pos.z,
            moon.vel.x, moon.vel.y, moon.vel.z
        );
    }
}


fn simulate(moons: Vec<Moon>, steps: i32) -> Vec<Moon> {
    let mut current = 0;
    let mut moons: Vec<Moon> = moons;
    let mut tmp: Vec<Moon>;
    while current < steps {
        // gravity
        tmp = vec![];

        loop {
            if let Some((moon1, rest)) = moons.split_first_mut() {
                if rest.len() > 0 {
                    for moon2 in rest.iter_mut() {
                        apply_gravity(&mut moon1.pos.x, &mut moon1.vel.x, &mut moon2.pos.x, &mut moon2.vel.x);
                        apply_gravity(&mut moon1.pos.y, &mut moon1.vel.y, &mut moon2.pos.y, &mut moon2.vel.y);
                        apply_gravity(&mut moon1.pos.z, &mut moon1.vel.z, &mut moon2.pos.z, &mut moon2.vel.z);
                    }
                }

                tmp.push(*moon1);
                moons = rest.to_vec();
            } else {
                break;
            }
        }

        moons = tmp;

        assert_eq!(moons.len(), 4);

        // velocity
        for moon in moons.iter_mut() {
            moon.pos.x += moon.vel.x;
            moon.pos.y += moon.vel.y;
            moon.pos.z += moon.vel.z;
        }

        current += 1;
    }
    moons

}


fn construct_moons() -> MyResult<Vec<Moon>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut moons: Vec<Moon> = vec![];
    for line in reader.lines() {
        let line = &line?;
        let pos: Position = serde_scan::scan!("<x={}, y={}, z={}>" <- line)?;
        let vel: Velocity = Velocity{x: 0, y: 0, z: 0};
        moons.push(Moon::new(pos, vel));
    }
    Ok(moons)
}

fn part1() -> MyResult<i32> {
    let moons = construct_moons()?;
    let moons = simulate(moons, 1000);
    Ok(moons.iter().map(|moon| moon.e_total()).sum::<i32>())
}

fn main() -> MyResult<()> {
    println!("Result Part 1: {:?}", part1()?);
    Ok(())
}