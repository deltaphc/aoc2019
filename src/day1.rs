use aoc_runner_derive::{aoc};

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|mass| mass / 3 - 2)
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|mass| {
            let mut running_mass = mass / 3 - 2;
            let mut total_mass = 0;
            while running_mass > 0 {
                total_mass += running_mass;
                running_mass = running_mass / 3 - 2;
            }
            total_mass
        })
        .sum()
}
