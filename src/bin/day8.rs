use itertools::Itertools;

#[allow(dead_code)]
const SAMPLE: &str = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

fn main() {
    let values: Vec<_> =
        // SAMPLE
        include_str!("../../input/day8")
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (unique, output) = l.split_once('|').unwrap();
                (unique.split_whitespace().collect_vec(), output.split_whitespace().collect_vec())
            })
            .collect();

    // let digits = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let p1 = {
        // 1 is 2, 4 is 4, 7 is 2,
        values.iter().map(|(_, o)| o).flatten()
            .filter(|x| [2, 4, 3, 7].contains(&x.len()))
            .count()
    };
    println!("p1 = {:?}", p1);

    let p2 = {

    };
    println!("p2 = {:?}", p2);
}
