use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn potential_energy(self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(self) -> i64 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn total_energy(self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn day12_gen(input: String) -> Vec<Moon> {
    input
        .trim()
        .lines()
        .map(|line| {
            let without_brackets = &line[1..(line.len() - 1)];
            let mut number_iter = without_brackets
                .split(',')
                .map(|component| {
                    let mut left_right = component.trim().split('=');
                    left_right.next();
                    let number = left_right.next().unwrap().parse::<i64>().unwrap();
                    number
                });

            let (x, y, z) = (number_iter.next().unwrap(), number_iter.next().unwrap(), number_iter.next().unwrap());
            Moon {
                pos: Vec3 { x, y, z },
                vel: Vec3::default(),
            }
        })
        .collect()
}

fn gravity_dir(current: i64, other: i64) -> i64 {
    match current.cmp(&other) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn simulation_step(moons: &mut [Moon]) {
    // Apply gravity
    for i in 0..moons.len() {
        let mut current_moon = moons[i];
        for j in 0..moons.len() {
            if j == i { continue; }
            let other_moon = moons[j];
            current_moon.vel += Vec3 {
                x: gravity_dir(current_moon.pos.x, other_moon.pos.x),
                y: gravity_dir(current_moon.pos.y, other_moon.pos.y),
                z: gravity_dir(current_moon.pos.z, other_moon.pos.z),
            };
            moons[i] = current_moon;
        }
    }

    // Apply velocity
    for i in 0..moons.len() {
        moons[i].pos += moons[i].vel;
    }
}

fn part1(moons: Vec<Moon>) -> i64 {
    let mut moons = moons;
    for _ in 0..1000 {
        simulation_step(&mut moons);
    }
    moons
        .iter()
        .map(|m| m.total_energy())
        .sum()
}

fn part2(moons: Vec<Moon>) -> u64 {
    let mut moons = moons;
    let initial_state = [moons[0], moons[1], moons[2], moons[3]];
    let mut num_steps = 0_u64;
    loop {
        simulation_step(&mut moons);
        let (num_steps_new, overflow) = num_steps.overflowing_add(1);
        if overflow {
            panic!("num_steps overflowed");
        }
        else {
            num_steps = num_steps_new;
        }
        let current_state = [moons[0], moons[1], moons[2], moons[3]];
        if current_state == initial_state { break; }
    }
    num_steps
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 12, day12_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
