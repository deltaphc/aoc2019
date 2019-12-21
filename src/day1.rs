fn part1(input: String) -> i32 {
    input
        .lines()
        .flat_map(|s| s.parse::<i32>())
        .map(|mass| mass / 3 - 2)
        .sum()
}

fn part2(input: String) -> i32 {
    input
        .lines()
        .flat_map(|s| s.parse::<i32>())
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

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new(2019, 1);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
