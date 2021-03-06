use std::collections::HashMap;

fn day6_gen(input: String) -> HashMap<String, String> {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(')');
            let (value, key) = (spl.next().unwrap(), spl.next().unwrap());
            (key.to_owned(), value.to_owned())
        })
        .collect()
}

fn part1(input: HashMap<String, String>) -> i32 {
    let direct_orbits = input.len() as i32;
    let mut total_indirect_orbits = 0;
    for (_, v) in &input {
        let mut indirect_orbits = 0;
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

fn part2(input: HashMap<String, String>) -> usize {
    let mut you_to_com = Vec::new();
    {
        let mut cursor = &input["YOU"];
        you_to_com.push(cursor);
        while let Some(next_val) = input.get(cursor) {
            if next_val != "COM" {
                you_to_com.push(next_val);
            }
            cursor = next_val;
        }
    }

    let mut san_to_com = Vec::new();
    {
        let mut cursor = &input["SAN"];
        san_to_com.push(cursor);
        while let Some(next_val) = input.get(cursor) {
            if next_val != "COM" {
                san_to_com.push(next_val);
            }
            cursor = next_val;
        }
    }

    let mut orbital_transfers = 0;
    for node in &you_to_com {
        orbital_transfers += 1;
        if let Some((i, _)) = san_to_com.iter().enumerate().find(|(_, item)| *item == node) {
            orbital_transfers += i - 1;
            break;
        }
    }
    orbital_transfers
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 6, day6_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
