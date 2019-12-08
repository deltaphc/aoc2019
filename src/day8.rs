use aoc_runner_derive::{aoc, aoc_generator};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
fn day8_gen(input: &str) -> Vec<u8> {
    input.trim().bytes().map(|digit_ascii| digit_ascii - 0x30).collect()
}

fn layer_count(input: &[u8]) -> usize {
    input.len() / (WIDTH * HEIGHT)
}

fn layer_data(input: &[u8], idx: usize) -> &[u8] {
    let base = idx * WIDTH * HEIGHT;
    &input[base..(base + WIDTH * HEIGHT)]
}

fn count_digits(input: &[u8], digit: u8) -> usize {
    input.iter().filter(|&&b| b == digit).count()
}

#[aoc(day8, part1)]
fn part1(input: &[u8]) -> usize {
    let mut fewest_zero_digits = usize::max_value();
    let mut fewest_zero_digits_idx = 0;
    for i in 0..layer_count(input) {
        let layer = layer_data(input, i);
        let num_zeroes = count_digits(layer, 0);
        if fewest_zero_digits > num_zeroes {
            fewest_zero_digits = num_zeroes;
            fewest_zero_digits_idx = i;
        }
    }

    let fewest_zero_layer = layer_data(input, fewest_zero_digits_idx);
    let num_one_digits = count_digits(fewest_zero_layer, 1);
    let num_two_digits = count_digits(fewest_zero_layer, 2);
    num_one_digits * num_two_digits
}
