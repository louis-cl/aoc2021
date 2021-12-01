use std::process::exit;

fn main() {
    let nums: Vec<_> = include_str!("../../input/day1").lines()
        .map(|line| line.parse::<i32>().unwrap()).collect();

    let mut last_num = nums[0];
    let mut count = 0;
    for n in &nums {
        if n > &last_num {
            count += 1;
        }
        last_num = *n;
    }

    println!("p1 = {:?}", count);

    let mut last_sum = i32::MAX;
    let mut sum_count = 0;
    for win in nums.windows(3) {
        let sum = win[0] + win[1] + win[2];
        if sum > last_sum {
            sum_count += 1;
        }
        last_sum = sum;
    }

    println!("p2 = {}", sum_count);
}
