use std::str::FromStr as _;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let masses: Vec<f32> = input
        .split_whitespace()
        .filter_map(|mass_str| f32::from_str(mass_str).ok())
        .collect();

    let module_fuel: Vec<f32> = masses
        .iter()
        .map(|&mass| (mass / 3.0).floor() - 2.0)
        .collect();

    module_fuel.iter().sum::<f32>() as i32
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let masses: Vec<f32> = input
        .split_whitespace()
        .filter_map(|mass_str| f32::from_str(mass_str).ok())
        .collect();

    let module_fuel: Vec<f32> = masses
        .iter()
        .map(|&mass| {
            let mut running_mass = (mass / 3.0).floor() - 2.0;
            let mut total_mass = 0.0;
            while running_mass > 0.0 {
                total_mass += running_mass;
                running_mass = (running_mass / 3.0).floor() - 2.0;
            }
            total_mass
        })
        .collect();

    module_fuel.iter().sum::<f32>() as i32
}
