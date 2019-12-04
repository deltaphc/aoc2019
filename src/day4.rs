use aoc_runner_derive::{aoc, aoc_generator};

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
                let digit: u32 = (num / 10_u32.pow(5 - i)) % 10;
                let prev_digit: u32 = (num / 10_u32.pow(5 - (i - 1))) % 10;
                if digit == prev_digit {
                    double_digit = true;
                }
                if digit < prev_digit {
                    never_decrease = false;
                    // Adding a break here hurts performance in part 1, but improves it in part 2... for some reason
                    // break;
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
                let digit: u32 = (num / 10_u32.pow(5 - i)) % 10;
                let prev_digit: u32 = (num / 10_u32.pow(5 - (i - 1))) % 10;
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
