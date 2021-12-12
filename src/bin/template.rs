#[allow(dead_code)]
const SAMPLE: &str = r#"

"#;

fn main() {
    let input =
        // SAMPLE
        include_str!("../../input/dayX")
            .lines()
            .filter(|l| !l.is_empty())
            .collect()
    ;

    let p1 = {

    };
    println!("p1 = {:?}", p1);

    let p2 = {

    };
    println!("p2 = {:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}