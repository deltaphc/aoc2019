use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation, IOReturn, ExecuteAction};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Black,
    White,
}

impl From<i64> for Tile {
    fn from(val: i64) -> Tile {
        match val {
            0 => Tile::Black,
            1 => Tile::White,
            _ => panic!("Invalid tile value {}", val),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum OutputMode {
    Paint,
    Turn,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[aoc_generator(day11)]
fn day11_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[i64]) -> usize {
    let mut prog = Program::from(input);
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut robot_dir = 0_i8; // 0 - Up, 1 - Right, 2 - Down, 3 - Left
    let mut output_mode = OutputMode::Paint;
    
    while !prog.is_halted() {
        prog.run(|io_op| {
            match io_op {
                IOOperation::Input => {
                    IOReturn::Input(
                        if tiles.contains_key(&(robot_x, robot_y)) {
                            match tiles[&(robot_x, robot_y)] {
                                Tile::Black => 0,
                                Tile::White => 1,
                            }
                        }
                        else {
                            0
                        }
                    )
                },
                IOOperation::Output(value) => {
                    match output_mode {
                        OutputMode::Paint => {
                            tiles.entry((robot_x, robot_y))
                                .and_modify(|tile| *tile = Tile::from(value))
                                .or_insert(Tile::from(value));
                            output_mode = OutputMode::Turn;
                        },
                        OutputMode::Turn => {
                            match value {
                                // Turn left
                                0 => robot_dir = (robot_dir - 1).rem_euclid(4),
                                // Turn right
                                1 => robot_dir = (robot_dir + 1).rem_euclid(4),
                                _ => panic!("Invalid direction output {}", value),
                            }

                            match robot_dir {
                                0 => { robot_y += 1; },
                                1 => { robot_x += 1; },
                                2 => { robot_y -= 1; },
                                3 => { robot_x -= 1; },
                                _ => panic!("Robot direction out of bounds: {}", robot_dir),
                            }

                            output_mode = OutputMode::Paint;
                        }
                    }
                    IOReturn::Output(ExecuteAction::Continue)
                },
            }
        });
    }
    tiles.len()
}

#[aoc(day11, part2)]
fn part2(input: &[i64]) -> String {
    let mut prog = Program::from(input);
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
    tiles.insert((0, 0), Tile::White);
    let mut min_width = 1;
    let mut max_width = 1;
    let mut min_height = 1;
    let mut max_height = 1;
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut robot_dir = 0_i8; // 0 - Up, 1 - Right, 2 - Down, 3 - Left
    let mut output_mode = OutputMode::Paint;
    let mut output_image = String::new();
    
    while !prog.is_halted() {
        prog.run(|io_op| {
            match io_op {
                IOOperation::Input => {
                    IOReturn::Input(
                        if tiles.contains_key(&(robot_x, robot_y)) {
                            match tiles[&(robot_x, robot_y)] {
                                Tile::Black => 0,
                                Tile::White => 1,
                            }
                        }
                        else {
                            0
                        }
                    )
                },
                IOOperation::Output(value) => {
                    match output_mode {
                        OutputMode::Paint => {
                            tiles.entry((robot_x, robot_y))
                                .and_modify(|tile| *tile = Tile::from(value))
                                .or_insert(Tile::from(value));
                            output_mode = OutputMode::Turn;
                        },
                        OutputMode::Turn => {
                            match value {
                                // Turn left
                                0 => robot_dir = (robot_dir - 1).rem_euclid(4),
                                // Turn right
                                1 => robot_dir = (robot_dir + 1).rem_euclid(4),
                                _ => panic!("Invalid direction output {}", value),
                            }

                            match robot_dir {
                                0 => { // up
                                    robot_y -= 1;
                                },
                                1 => { // right
                                    robot_x += 1;
                                },
                                2 => { // down
                                    robot_y += 1;
                                },
                                3 => { // left
                                    robot_x -= 1;
                                },
                                _ => panic!("Robot direction not within 0-3: {}", robot_dir),
                            }

                            min_width = robot_x.min(min_width);
                            max_width = robot_x.max(max_width);
                            min_height = robot_y.min(min_height);
                            max_height = robot_y.max(max_height);

                            output_mode = OutputMode::Paint;
                        }
                    }
                    IOReturn::Output(ExecuteAction::Continue)
                },
            }
        });
    }

    output_image.push('\n');
    for y in min_height..=max_height {
        for x in min_width..=max_width {
            if !tiles.contains_key(&(x, y)) {
                output_image.push(' ');
                continue;
            }
            output_image.push(
                match tiles[&(x, y)] {
                    Tile::Black => ' ',
                    Tile::White => '*',
                }
            );
        }
        output_image.push('\n');
    }

    output_image
}
