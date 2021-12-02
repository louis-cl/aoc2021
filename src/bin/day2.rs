use itertools::Itertools;
use crate::Command::{Down, Forward, Up};

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}

fn main() {
    let lines: Vec<_> = include_str!("../../input/day2").lines().collect();

    let input: Vec<_> = lines.iter()
        .map(|line| {
            let (comm, amount) = line.split_once(" ").unwrap();
            let n = amount.parse().unwrap();
            match comm {
                "forward" => Forward(n),
                "up" => Up(n),
                "down" => Down(n),
                &_ => panic!()
            }
        }).collect();

    let p1 = {
        let (mut x, mut d) = (0, 0);
        for c in input {
            match c {
                Forward(n) =>  x += n,
                Down(n) => d += n,
                Up(n) => d -= n
            }
        }
        x * d
    };
    println!("p1 = {:?}", p1);

    // println!("p2 = {}", p2);
}
