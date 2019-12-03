use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

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
    OccupiedBy(usize),
    Intersection,
}

fn process_cell(wire_grid: &mut HashMap<(i32, i32), GridCell>, x: i32, y: i32, current_wire_idx: usize) {
    wire_grid
        .entry((x, y))
        .and_modify(|cell| {
            if let GridCell::OccupiedBy(idx) = cell {
                if *idx != current_wire_idx { *cell = GridCell::Intersection; }
            }
        })
        .or_insert(GridCell::OccupiedBy(current_wire_idx));
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
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
    
    let mut wire_grid: HashMap<(i32, i32), GridCell> = HashMap::new();
    wire_grid.insert((0, 0), GridCell::Origin);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (wire_idx, wire) in wires.iter().enumerate() {
        for instr in wire {
            match instr {
                WireInstruction::Up(amt) => {
                    for y_step in y..=(y + amt) {
                        process_cell(&mut wire_grid, x, y_step, wire_idx);
                    }
                    y += amt;
                },
                WireInstruction::Down(amt) => {
                    for y_step in ((y - amt)..=y).rev() {
                        process_cell(&mut wire_grid, x, y_step, wire_idx);
                    }
                    y -= amt;
                },
                WireInstruction::Left(amt) => {
                    for x_step in ((x - amt)..=x).rev() {
                        process_cell(&mut wire_grid, x_step, y, wire_idx);
                    }
                    x -= amt;
                },
                WireInstruction::Right(amt) => {
                    for x_step in x..=(x + amt) {
                        process_cell(&mut wire_grid, x_step, y, wire_idx);
                    }
                    x += amt;
                },
            }
        }
    }

    let mut closest_distance = i32::max_value();
    for ((x, y), cell) in &wire_grid {
        if let GridCell::Intersection = cell {
            if x.abs() + y.abs() < closest_distance {
                closest_distance = x.abs() + y.abs();
            }
        }
    }
    closest_distance
}