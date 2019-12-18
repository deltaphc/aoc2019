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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new(2019, 1);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
