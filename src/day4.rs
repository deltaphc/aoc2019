use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let mut split_iter = input.split('-').flat_map(|num_str| num_str.parse::<u32>());
    let (lower, upper) = (split_iter.next().unwrap(), split_iter.next().unwrap());

    (lower..=upper)
        .filter(|&num| {
            let num_str = num.to_string();
            let mut double_digit = false;
            let mut never_decrease = true;
            for i in 1..num_str.len() {
                let ch = char::from_str(&num_str[i..=i]).unwrap();
                let prev_ch = char::from_str(&num_str[(i - 1)..=(i - 1)]).unwrap();
                if ch == prev_ch {
                    double_digit = true;
                }
                if ch < prev_ch {
                    never_decrease = false;
                }
            }
            double_digit && never_decrease
        })
        .count()
}
