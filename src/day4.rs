use aoc_runner_derive::{aoc, aoc_generator};

const POW10: [u32; 6] = [1, 10, 100, 1000, 10000, 100000];

/// Returns the `n`th digit (from left to right) of the given six-digit `num`.
fn nth_digit(num: u32, n: usize) -> u32 {
    (num / POW10[5 - n]) % 10
}

/// Returns a six-element array containing the rightmost six digits of `num`.
fn digits(num: u32) -> [u32; 6] {
    [
        nth_digit(num, 0),
        nth_digit(num, 1),
        nth_digit(num, 2),
        nth_digit(num, 3),
        nth_digit(num, 4),
        nth_digit(num, 5),
    ]
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
            for digit_pair in digits(num).windows(2) {
                let (prev_digit, digit) = (digit_pair[0], digit_pair[1]);

                if digit < prev_digit {
                    return false;
                }
                
                if digit == prev_digit {
                    double_digit = true;
                }
            }
            double_digit
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(&(lower, upper): &(u32, u32)) -> usize {
    (lower..=upper)
        .filter(|&num| {
            let mut double_digit = false;
            let mut num_matches = 0;
            for digit_pair in digits(num).windows(2) {
                let (prev_digit, digit) = (digit_pair[0], digit_pair[1]);

                if digit < prev_digit {
                    return false;
                }

                if digit == prev_digit {
                    num_matches += 1;
                }
                else {
                    if num_matches == 1 {
                        double_digit = true;
                    }
                    num_matches = 0;
                }
            }

            // Accounting for leftover matches after digit loop finishes
            if num_matches == 1 {
                double_digit = true;
            }
            double_digit
        })
        .count()
}
