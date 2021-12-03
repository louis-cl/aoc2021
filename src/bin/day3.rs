use std::collections::HashMap;
use itertools::Itertools;

const SAMPLE: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

fn read_test(input: &str) -> impl Iterator<Item=&str> {
    input.lines().filter(|line| !line.is_empty())
}

fn o2(lines: Vec<Vec<char>>, i: usize) -> String {
    if lines.len() == 1 {
        let r: String = lines[0].iter().collect();
        return r[i..].to_string();
    }
    if lines[0].len() <= i {
        return String::from("");
    }

    let n = lines.len();
    let ones = lines.iter().filter(|c| c[i] == '1').count();
    let zeros = n - ones;

    if ones >= zeros {
        // keep 1
        let rest = o2(
            lines.into_iter().filter(| c | c[i] == '1').collect(),
            i+1);
        String::from("1") + rest.as_str()
    } else {
        // keep 0
        let rest = o2(
            lines.into_iter().filter(| c | c[i] == '0').collect(),
            i+1);
        String::from("0") + rest.as_str()
    }
}

fn co2(lines: Vec<Vec<char>>, i: usize) -> String {
    if lines.len() == 1 {
        let r: String = lines[0].iter().collect();
        return r[i..].to_string();
    }

    if lines[0].len() <= i {
        return String::from("");
    }

    let n = lines.len();
    let ones = lines.iter().filter(|c| c[i] == '1').count();
    let zeros = n - ones;

    if ones < zeros {
        // keep 1
        let rest = co2(
            lines.into_iter().filter(| c | c[i] == '1').collect(),
            i+1);
        String::from("1") + rest.as_str()
    } else {
        // keep 0
        let rest = co2(
            lines.into_iter().filter(| c | c[i] == '0').collect(),
            i+1);
        String::from("0") + rest.as_str()
    }
}

fn main() {
    // let lines: Vec<_> = include_str!("../../input/day3").lines().collect();
    let lines: Vec<_> =
        // read_test(SAMPLE)
        include_str!("../../input/day3").lines()
        .map(|x| x.to_string())
        .collect();

    let p1 = {
        let mut zeroCount = HashMap::new();
        let mut oneCount = HashMap::new();

        let n = lines[0].len();

        for l in &lines {
            for (i, c) in l.chars().enumerate() {
                if c == '0' {
                    *zeroCount.entry(i).or_insert(0) += 1;
                } else {
                    *oneCount.entry(i).or_insert(0) += 1;
                }
            }
        }

        let mut gamma = String::new();
        let mut eps = String::new();

        for i in 0..n {
            let z = zeroCount.get(&i).unwrap_or(&0);
            let o = oneCount.get(&i).unwrap_or(&0);
            if z > o {
                gamma.push('0');
                eps.push('1');
            } else {
                gamma.push('1');
                eps.push('0');
            }
        }

        let g = i64::from_str_radix(gamma.as_str(), 2).unwrap();
        let e = i64::from_str_radix(eps.as_str(), 2).unwrap();

        println!("{:?} {:?}", g, e);
        g * e
    };

    println!("p1 = {:?}", p1);

    let p2 = {
        let chars: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
        let o2= o2(chars.clone(), 0);
        let co2= co2(chars, 0);
        println!("o2 = {}, co2 = {}", o2, co2);
        i32::from_str_radix(o2.as_str(), 2).unwrap() *
            i32::from_str_radix(co2.as_str(), 2).unwrap()
    };
    println!("p2 = {}", p2);
}
