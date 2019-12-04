use aoc_runner_derive::{aoc, aoc_generator};

const POW10: [u32; 6] = [1, 10, 100, 1000, 10000, 100000];

/// Returns the `n`th digit (from left to right) of the given six-digit `num`.
#[inline(always)]
fn nth_digit(num: u32, n: usize) -> u32 {
    (num / POW10[5 - n]) % 10
}

#[aoc_generator(day4)]
fn day4_gen(input: &str) -> (u32, u32) {
    (
        input[0..6].parse::<u32>().unwrap(),
        input[7..13].parse::<u32>().unwrap()
    )
}

#[aoc(day4, part1)]
fn part1(&(lower, upper): &(u32, u32)) -> usize {
    (lower..=upper)
        .filter(|&num| {
            let mut double_digit = false;
            let mut never_decrease = true;
            for i in 1..6 { // i is indexing digits from left to right
                let digit: u32 = nth_digit(num, i);
                let prev_digit: u32 = nth_digit(num, i - 1);
                if digit == prev_digit {
                    double_digit = true;
                }
                if digit < prev_digit {
                    never_decrease = false;
                    break;
                }
            }
            double_digit && never_decrease
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(&(lower, upper): &(u32, u32)) -> usize {
    (lower..=upper)
        .filter(|&num| {
            let mut double_digit = false;
            let mut never_decrease = true;
            let mut num_matches = 0;
            for i in 1..6 { // i is indexing digits from left to right
                let digit: u32 = nth_digit(num, i);
                let prev_digit: u32 = nth_digit(num, i - 1);
                if digit == prev_digit {
                    num_matches += 1;
                }
                else {
                    if num_matches == 1 {
                        double_digit = true;
                    }
                    num_matches = 0;
                }
                if digit < prev_digit {
                    never_decrease = false;
                    break;
                }
            }

            // Accounting for leftover matches after digit loop finishes
            if num_matches == 1 {
                double_digit = true;
            }
            
            double_digit && never_decrease
        })
        .count()
}