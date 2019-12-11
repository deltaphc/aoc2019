use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, BTreeMap};
use std::cmp::Ordering;

const PI: f32 = std::f32::consts::PI;
const TWO_PI: f32 = 2.0 * PI;
const PI_OVER_TWO: f32 = std::f32::consts::FRAC_PI_2;

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
fn part2(input: &[(f32, f32)]) -> i32 { // currently incorrect
    let (station_idx, _) = find_ideal_asteroid(input);
    let (station_x, station_y) = input[station_idx];
    let mut asteroids: Vec<Option<(f32, f32)>> = input.iter().copied().map(Some).collect();
    //let mut targets: BTreeMap<i32, usize> = BTreeMap::new(); // angle -> index within asteroids
    let mut total_destroyed = 0;

    asteroids.sort_unstable_by(|a, b| {
        let (a_x, a_y) = a.unwrap();
        let (b_x, b_y) = b.unwrap();
        let dist_to_a = f32::hypot(station_x - a_x, station_y - a_y);
        let dist_to_b = f32::hypot(station_x - b_x, station_y - b_y);
        let ang_to_a = {
            let mut ang = (station_y - a_y).atan2(station_x - a_x).rem_euclid(TWO_PI);
            if ang >= 0.0 && ang < PI_OVER_TWO {
                ang += TWO_PI;
            }
            ang
        };
        let ang_to_b = {
            let mut ang = (station_y - b_y).atan2(station_x - b_x).rem_euclid(TWO_PI);
            if ang >= 0.0 && ang < PI_OVER_TWO {
                ang += TWO_PI;
            }
            ang
        };
        let (ang_to_a, ang_to_b) = ((ang_to_a * 1000.0) as i32, (ang_to_b * 1000.0) as i32);

        match ang_to_a.cmp(&ang_to_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                dist_to_a.partial_cmp(&dist_to_b).unwrap_or(Ordering::Equal)
            }
        }
    });

    while asteroids.len() > 0 {
        for i in (0..asteroids.len()).rev() {
            let asteroid = asteroids[i];
            if asteroid.is_none() { continue; }
            let (ast_x, ast_y) = asteroid.unwrap();
            if ast_x == station_x && ast_y == station_y { continue; }
            let (ast_x, ast_y) = (ast_x as i32, ast_y as i32);

            asteroids[i] = None;
            total_destroyed += 1;
            if total_destroyed == 200 {
                return ast_x * 100 + ast_y;
            }
        }
    }

    /* while asteroids.len() > 0 {
        targets.clear();
        for i in 0..asteroids.len() {
            if i == station_idx { continue; }
            if asteroids[i].is_none() { continue; }
            
            let (current_x, current_y) = asteroids[i].unwrap();
            let angle = (current_y - station_y).atan2(current_x - station_x).rem_euclid(TWO_PI);

            // Account for the first targets being north of the station
            let adjusted_angle = {
                if angle >= 0.0 && angle < PI_OVER_TWO {
                    angle + TWO_PI
                }
                else {
                    angle
                }
            };
            
            let angle_int = (adjusted_angle * 10000.0) as i32;
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

        for (_, &asteroids_idx) in targets.iter().rev() {
            let target_asteroid = asteroids[asteroids_idx];
            let (ast_x, ast_y) = target_asteroid.unwrap();
            let (ast_x, ast_y) = (ast_x as i32, ast_y as i32);
            asteroids[asteroids_idx] = None;
            total_destroyed += 1;
            if total_destroyed == 200 {
                return ast_x * 100 + ast_y;
            }
        }
    } */
    -6969
}
