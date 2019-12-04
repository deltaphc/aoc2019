use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type WireGridMap = HashMap<(i32, i32), GridCell>;

#[derive(Debug, Copy, Clone)]
enum WireInstruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridCell {
    Origin,
    OccupiedBy { wire_idx: usize, at_step: u32 },
    Intersection([u32; 2]),
}

#[aoc_generator(day3)]
fn day3_gen(input: &str) -> WireGridMap {
    let wires: Vec<Vec<WireInstruction>> = input
        .lines()
        .map(|line| {
            line
                .split(',')
                .map(|instr| {
                    let num = instr[1..].parse::<i32>().unwrap();
                    match &instr[0..=0] {
                        "U" => WireInstruction::Up(num),
                        "D" => WireInstruction::Down(num),
                        "L" => WireInstruction::Left(num),
                        "R" => WireInstruction::Right(num),
                        _ => panic!("Unknown wire instruction: {}", instr),
                    }
                })
                .collect()
        })
        .collect();
    
    let mut wire_grid: WireGridMap = WireGridMap::new();
    wire_grid.insert((0, 0), GridCell::Origin);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut steps = [0_u32; 2];
    for (wire_idx, wire) in wires.iter().enumerate() {
        for instr in wire {
            match instr {
                WireInstruction::Up(amt) => {
                    for y_step in y..=(y + amt) {
                        if y_step > y { steps[wire_idx] += 1; }
                        process_cell(&mut wire_grid, x, y_step, wire_idx, steps);
                    }
                    y += amt;
                },
                WireInstruction::Down(amt) => {
                    for y_step in ((y - amt)..=y).rev() {
                        if y_step < y { steps[wire_idx] += 1; }
                        process_cell(&mut wire_grid, x, y_step, wire_idx, steps);
                    }
                    y -= amt;
                },
                WireInstruction::Left(amt) => {
                    for x_step in ((x - amt)..=x).rev() {
                        if x_step < x { steps[wire_idx] += 1; }
                        process_cell(&mut wire_grid, x_step, y, wire_idx, steps);
                    }
                    x -= amt;
                },
                WireInstruction::Right(amt) => {
                    for x_step in x..=(x + amt) {
                        if x_step > x { steps[wire_idx] += 1; }
                        process_cell(&mut wire_grid, x_step, y, wire_idx, steps);
                    }
                    x += amt;
                },
            }
        }
        x = 0; y = 0;
    }

    wire_grid
}

fn process_cell(wire_grid: &mut WireGridMap, x: i32, y: i32, current_wire_idx: usize, wire_steps: [u32; 2]) {
    wire_grid
        .entry((x, y))
        .and_modify(|cell| {
            match cell {
                GridCell::OccupiedBy { wire_idx, at_step } => {
                    if *wire_idx != current_wire_idx { // By this point we're on the second wire
                        // We don't use wire_steps[0] here because it would contain the *total* steps
                        // for the first wire, but we only want the steps up to this point
                        *cell = GridCell::Intersection([*at_step, wire_steps[1]]);
                    }
                },
                _ => (),
            }
        })
        .or_insert(GridCell::OccupiedBy { wire_idx: current_wire_idx, at_step: wire_steps[current_wire_idx] });
}

#[aoc(day3, part1)]
fn part1(input: &WireGridMap) -> i32 {
    let mut closest_distance = i32::max_value();
    for ((x, y), cell) in input {
        if let GridCell::Intersection(_) = cell {
            let manhattan_dist = x.abs() + y.abs();
            if manhattan_dist < closest_distance {
                closest_distance = manhattan_dist;
            }
        }
    }
    closest_distance
}

#[aoc(day3, part2)]
fn part2(input: &WireGridMap) -> u32 {
    let mut lowest_step_sum = u32::max_value();
    for ((_, _), cell) in input {
        if let GridCell::Intersection(steps) = cell {
            let step_sum = steps[0] + steps[1];
            if step_sum < lowest_step_sum {
                lowest_step_sum = step_sum;
            }
        }
    }
    lowest_step_sum
}
