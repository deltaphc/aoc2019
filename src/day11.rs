use crate::intcode::prelude::*;
use std::collections::HashMap;

mod dir {
    pub const UP: i8 = 0;
    pub const RIGHT: i8 = 1;
    pub const DOWN: i8 = 2;
    pub const LEFT: i8 = 3;
}

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

/// Runs the hull painting robot and returns a tuple containing the number of tiles painted at least once, and a printable String with the output image, respectively.
fn paint_hull(input: &[i64], starting_tile: Tile) -> (usize, String) {
    let mut prog = Program::from(input);
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
    tiles.insert((0, 0), starting_tile);
    let mut min_width = 1;
    let mut max_width = 1;
    let mut min_height = 1;
    let mut max_height = 1;
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut robot_dir = dir::UP;
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
                                dir::UP => { robot_y -= 1; },
                                dir::RIGHT => { robot_x += 1; },
                                dir::DOWN => { robot_y += 1; },
                                dir::LEFT => { robot_x -= 1; },
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
                    Tile::White => '\u{2588}',
                }
            );
        }
        output_image.push('\n');
    }

    (tiles.len(), output_image)
}

fn day11_gen(input: String) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

fn part1(input: Vec<i64>) -> usize {
    let (tiles_painted, _) = paint_hull(&input, Tile::Black);
    tiles_painted
}

fn part2(input: Vec<i64>) -> String {
    let (_, output_image) = paint_hull(&input, Tile::White);
    output_image
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 11, day11_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
