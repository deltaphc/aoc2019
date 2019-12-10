use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day10)]
fn day10_gen(input: &str) -> Vec<(f32, f32)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().bytes().enumerate().filter_map(move |(x, b)| {
                match b {
                    b'#' => Some((x as f32 + 0.5, y as f32 + 0.5)),
                    _ => None,
                }
            })
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[(f32, f32)]) -> usize {
    let mut most_asteroids = 0;
    let mut angles = HashSet::new();

    for i in 0..input.len() {
        let (current_x, current_y) = input[i];
        angles.clear();
        for j in 0..input.len() {
            if j == i { continue; }
            let (other_x, other_y) = input[j];
            let angle = (other_y - current_y).atan2(current_x - other_x);
            let angle_deg = (angle.to_degrees() * 1000.0) as i32;
            angles.insert(angle_deg);
        }
        if angles.len() > most_asteroids {
            most_asteroids = angles.len();
        }
    }

    most_asteroids
}
