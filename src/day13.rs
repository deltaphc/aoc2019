use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation, IOReturn, ExecuteAction};
use std::collections::HashMap;

#[aoc_generator(day13)]
fn day13_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[i64]) -> usize {
    let mut prog = Program::from(input);
    let mut game_screen: HashMap<(i64, i64), i64> = HashMap::new(); // (x, y) -> tile
    let mut output_select = 0; // 0 = x, 1 = y, 2 = tile
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut tile = 0_i64;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(0),
            IOOperation::Output(value) => {
                match output_select {
                    0 => x = value,
                    1 => y = value,
                    2 => {
                        tile = value;
                        game_screen.entry((x, y))
                            .and_modify(|t| *t = tile)
                            .or_insert(tile);
                    },
                    _ => unreachable!(),
                }
                output_select = (output_select + 1) % 3;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    game_screen.iter().filter(|(_, &t)| t == 2).count()
}

fn find_tile(game_screen: &HashMap<(i64, i64), i64>, tile: i64) -> (i64, i64) {
    let ((x, y), _) = game_screen.iter().find(|(_, &t)| t == tile).unwrap();
    (*x, *y)
}

#[aoc(day13, part2)]
fn part2(input: &[i64]) -> i64 {
    let mut prog = Program::from(input);
    prog.prog_mut()[0] = 2; // free play
    let mut game_screen: HashMap<(i64, i64), i64> = HashMap::new(); // (x, y) -> tile
    let mut output_select = 0; // 0 = x, 1 = y, 2 = tile/score
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut tile = 0_i64;
    let mut score = 0_i64;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => {
                let (ball_x, _) = find_tile(&game_screen, 4);
                let (paddle_x, _) = find_tile(&game_screen, 3);
                let mut joystick = 0_i64;

                if ball_x < paddle_x {
                    joystick = -1;
                }
                else if ball_x > paddle_x {
                    joystick = 1;
                }

                IOReturn::Input(joystick)
            },
            IOOperation::Output(value) => {
                match output_select {
                    0 => x = value,
                    1 => y = value,
                    2 => {
                        if x == -1 && y == 0 {
                            score = value;
                        }
                        else {
                            tile = value;
                            game_screen.entry((x, y))
                                .and_modify(|t| *t = tile)
                                .or_insert(tile);
                        }
                    },
                    _ => unreachable!(),
                }
                output_select = (output_select + 1) % 3;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    score
}
