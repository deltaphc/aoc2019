// WIP

fn base_pattern(pos: usize) -> Vec<i8> {
    let base = [0, 1, 0, -1];
    let mut pattern: Vec<i8> = base
        .iter()
        .flat_map(|&n| std::iter::repeat(n).take(pos + 1))
        .collect();
    pattern.remove(0);
    pattern
}

fn fft(digits: &[i8]) -> Vec<i8> {
    digits
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let pattern = base_pattern(i);
            let mut pattern_iter = pattern.iter().copied().cycle();
            digits
                .iter()
                .enumerate()
                .map(|(j, d2)| (*d2 as i8) * pattern_iter.next().unwrap())
                .sum()
        })
        .collect()
}

fn day16_gen(input: String) -> Vec<i8> {
    input
        .bytes()
        .map(|b| (b - b'0') as i8)
        .collect()
}

fn part1(input: Vec<i8>) -> String {
    let mut work_digits = input[0..8].to_vec();
    for _ in 0..100 {
        work_digits = fft(&work_digits);
    }
    "".to_string()
}

fn part2(_input: Vec<i8>) -> String {
    "".to_string()
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 16, day16_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
