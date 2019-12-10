use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, BTreeMap};

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

/// Finds the asteroid with the most number of detectable asteroids and returns (`index`, `asteroids_detected`).
fn find_ideal_asteroid(asteroids: &[(f32, f32)]) -> (usize, usize) {
    let mut most_asteroids = 0;
    let mut most_asteroids_idx = 0;
    let mut angles = HashSet::new();

    for i in 0..asteroids.len() {
        let (current_x, current_y) = asteroids[i];
        angles.clear();
        for j in 0..asteroids.len() {
            if j == i { continue; }
            let (other_x, other_y) = asteroids[j];
            let angle = (other_y - current_y).atan2(other_x - current_x);
            let angle_int = (angle * 1000.0) as i32;
            angles.insert(angle_int);
        }
        if angles.len() > most_asteroids {
            most_asteroids = angles.len();
            most_asteroids_idx = i;
        }
    }

    (most_asteroids_idx, most_asteroids)
}

#[aoc(day10, part1)]
fn part1(input: &[(f32, f32)]) -> usize {
    let (_, asteroids_detected) = find_ideal_asteroid(input);
    asteroids_detected
}

#[aoc(day10, part2)]
fn part2(input: &[(f32, f32)]) -> i32 {
    let (station_idx, _) = find_ideal_asteroid(input);
    let (station_x, station_y) = input[station_idx];
    let mut asteroids: Vec<Option<(f32, f32)>> = input.iter().copied().map(Some).collect();
    let mut targets: BTreeMap<i32, usize> = BTreeMap::new(); // angle -> index within asteroids
    let mut total_destroyed = 0;

    while asteroids.len() > 0 {
        targets.clear();
        for i in 0..asteroids.len() {
            if i == station_idx { continue; }
            if asteroids[i].is_none() { continue; }
            let (current_x, current_y) = asteroids[i].unwrap();
            let angle = (current_y - station_y).atan2(current_x - station_x);

            // Account for the first targets being north of the station
            let adjusted_angle = angle.rem_euclid(std::f32::consts::FRAC_2_PI) - std::f32::consts::FRAC_PI_2;
            
            let angle_int = (adjusted_angle * 1000.0) as i32;
            targets.entry(angle_int)
                .and_modify(|idx| {
                    let (target_x, target_y) = asteroids[*idx].unwrap();
                    let current_dist = f32::hypot(current_x - station_x, current_y - station_y);
                    let target_dist = f32::hypot(target_x - station_x, target_y - station_y);
                    if current_dist < target_dist {
                        *idx = i;
                    }
                })
                .or_insert(i);
        }

        for (_, &asteroids_idx) in targets.iter() {
            let target_asteroid = asteroids[asteroids_idx];
            let (ast_x, ast_y) = target_asteroid.unwrap();
            let (ast_x, ast_y) = (ast_x as i32, ast_y as i32);
            asteroids[asteroids_idx] = None;
            total_destroyed += 1;
            if total_destroyed == 200 {
                return ast_x * 100 + ast_y;
            }
        }
    }
    -6969
}
