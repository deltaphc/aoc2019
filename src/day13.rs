use crate::intcode::prelude::*;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

fn day13_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

fn part1(input: Vec<i64>) -> usize {
    let mut prog = Program::from(input.as_slice());
    let mut game_screen: HashMap<(i64, i64), i64> = HashMap::new(); // (x, y) -> tile
    let mut output_select = 0_u8; // 0 = x, 1 = y, 2 = tile
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
                    _ => unsafe { unreachable_unchecked() },
                }
                output_select = (output_select + 1) % 3;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    game_screen.iter().filter(|(_, &t)| t == 2).count()
}

fn part2(input: Vec<i64>) -> i64 {
    let mut prog = Program::from(input.as_slice());
    prog.prog_mut()[0] = 2; // free play
    let mut game_screen: HashMap<(i64, i64), i64> = HashMap::new(); // (x, y) -> tile
    let mut output_select = 0_u8; // 0 = x, 1 = y, 2 = tile/score
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut tile = 0_i64;
    let mut score = 0_i64;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => {
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
                            if tile == 4 {
                                ball_x = x;
                            }
                            else if tile == 3 {
                                paddle_x = x;
                            }
                            game_screen.entry((x, y))
                                .and_modify(|t| *t = tile)
                                .or_insert(tile);
                        }
                    },
                    _ => unsafe { unreachable_unchecked() },
                }
                output_select = (output_select + 1) % 3;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    score
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new_with_serializer(2019, 13, day13_gen);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
