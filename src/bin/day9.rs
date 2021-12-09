use std::collections::{HashMap};

#[allow(dead_code)]
const SAMPLE: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

fn main() {
    let values: Vec<Vec<u32>> =
        // SAMPLE
        include_str!("../../input/day9")
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    ;

    let map = values.into_iter().enumerate()
        .map(|(i, v)|
            v.into_iter().enumerate().map(move |(j, e)| (i, j, e)))
        .flatten()
        .fold(HashMap::new(), |mut acc, (i, j, e)| {
            acc.insert((i as i32, j as i32), e);
            acc
        });

    let p1 = {
        let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
        map.iter()
            .filter(|((x,y),v)| {
                for (dx,dy) in dirs {
                    if map.get(&(x + dx, y + dy)).unwrap_or(&10) <= v {
                        return false;
                    }
                }
                true
            })
            .map(|((_,_), v)| v+1)
            .sum::<u32>()
    };
    println!("p1 = {:?}", p1);

    let p2 = {

    };
    println!("p2 = {:?}", p2);
}
