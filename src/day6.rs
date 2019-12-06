use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day6)]
fn day6_gen(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(')');
            let (value, key) = (spl.next().unwrap(), spl.next().unwrap());
            (key.to_owned(), value.to_owned())
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &HashMap<String, String>) -> i32 {
    let direct_orbits = input.len() as i32;
    let mut total_indirect_orbits = 0;
    for (k, v) in input {
        let mut indirect_orbits = -1; // we don't count the immediate next orbit since that's direct
        let mut current_v = v;
        while let Some(next_val) = input.get(current_v) {
            indirect_orbits += 1;
            if next_val == "COM" { break; }
            current_v = next_val;
        }
        total_indirect_orbits += indirect_orbits;
    }
    
    direct_orbits + total_indirect_orbits
}
