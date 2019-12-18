mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
// mod day15;
mod day16;

const DAY_RUNNERS: [fn () -> Result<(), Box<dyn std::error::Error>>; 25] = [
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    day7::run,
    day8::run,
    day9::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    empty_run,
    day16::run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
    empty_run,
];

fn empty_run() -> Result<(), Box<dyn std::error::Error>> { unimplemented!() }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next(); // ignore executable path
    let day = args.next().unwrap().parse::<usize>()?;
    DAY_RUNNERS[day - 1]()?;
    Ok(())
}
