#[derive(Debug, Copy, Clone)]
enum Instruction {
    DealIntoNewStack,
    DealWithIncrement(i32),
    Cut(i32),
}

fn gen(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.starts_with("deal with increment ") {
                Instruction::DealWithIncrement(line[20..].parse::<i32>().unwrap())
            }
            else if line.starts_with("cut ") {
                Instruction::Cut(line[4..].parse::<i32>().unwrap())
            }
            else if line.starts_with("deal into new stack") {
                Instruction::DealIntoNewStack
            }
            else {
                panic!("Unknown shuffle instruction: {}", line)
            }
        })
        .collect()
}

fn deal_into_new_stack(deck: &mut [i32]) {
    deck.reverse();
}

fn cut(deck: &mut [i32], len: i32) {
    if len > 0 {
        deck.rotate_left(len as usize);
    }
    else {
        deck.rotate_right(len.abs() as usize);
    }
}

fn deal_with_increment(deck: &mut [i32], inc: i32) {
    let mut new_deck = vec![-1; deck.len()];
    let mut new_deck_idx = 0;
    for card in deck.iter().copied() {
        new_deck[new_deck_idx] = card;
        new_deck_idx = (new_deck_idx + inc as usize) % deck.len();
    }
    deck.copy_from_slice(&new_deck);
}

fn part1(input: Vec<Instruction>) -> usize {
    let mut deck: Vec<i32> = (0..10007).collect();
    for inst in &input {
        match inst {
            Instruction::DealIntoNewStack => deal_into_new_stack(&mut deck),
            Instruction::Cut(len) => cut(&mut deck, *len),
            Instruction::DealWithIncrement(inc) => deal_with_increment(&mut deck, *inc),
        }
    }
    let (result, _) = deck.iter().enumerate().find(|(_, &card)| card == 2019).unwrap();
    result
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 22, gen);
    let part1 = Puzzle::new(1, part1);
    //let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    //day.run(&part2)?;
    Ok(())
}
