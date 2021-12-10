use crate::State::{Complete, Corrupted, Incomplete};

#[allow(dead_code)]
const SAMPLE: &str = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

#[derive(Debug)]
enum State {
    Incomplete,
    Corrupted(u32),
    Complete,
}

fn main() {
    let lines: Vec<_> =
        // SAMPLE
        include_str!("../../input/day10")
            .lines()
            .filter(|l| !l.is_empty())
            .collect()
    ;

    fn points(c: &char) -> u32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("wtf is {}", c)
        }
    }

    fn matching(o: &char, c: &char) -> bool {
        match (o,c) {
            ('(',')') | ('[',']') | ('{','}') | ('<','>') => true,
            _ => false
        }
    }

    let p1 = {
        lines.iter().map(|line| {
            let mut stack = Vec::with_capacity(line.len());
            for c in line.chars() {
                match c {
                    '('|'['|'{'|'<' => stack.push(c),
                    ')'|']'|'}'|'>' => {
                        match stack.last() {
                            None => return Incomplete,
                            Some(o) => {
                                if matching(o, &c) {
                                    stack.pop();
                                } else {
                                    return Corrupted(points(&c));
                                }
                            }
                        }
                    },
                    _ => panic!("unknown char")
                }
            }
            if stack.is_empty() {
                return Complete;
            }
            Incomplete
        }).map(|s| match s {
            Corrupted(s) => s,
            _ => 0
        }).sum::<u32>()
    };
    println!("p1 = {:?}", p1);

    let p2 = {

    };
    println!("p2 = {:?}", p2);
}
