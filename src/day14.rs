// WIP

use std::collections::HashMap;

type ChemMap = HashMap<String, (u32, Vec<(u32, String)>)>;

fn day14_gen(input: &str) -> ChemMap {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_split_iter = line.trim().split(" => ");
            let chem_list_iter = line_split_iter.next().unwrap().split(", ");
            let mut output_chem_iter = line_split_iter.next().unwrap().split(' ');
            
            let chem_key_amt = output_chem_iter.next().unwrap().parse::<u32>().unwrap();
            let chem_key = output_chem_iter.next().unwrap().to_owned();
            let chem_list: Vec<(u32, String)> = chem_list_iter
                .map(|chem_str| {
                    let mut chem_str_iter = chem_str.split(' ');
                    let chem_amt = chem_str_iter.next().unwrap().parse::<u32>().unwrap();
                    let chem = chem_str_iter.next().unwrap().to_owned();
                    (chem_amt, chem)
                })
                .collect();
            
            (chem_key, (chem_key_amt, chem_list))
        })
        .collect()
}

fn total_ore(map: &ChemMap, chem: &str, amt_needed: u32) -> u32 {
    let (chem_amt, chem_list) = &map[chem];
    let chem_amt = *chem_amt;
    let mut ore_total = 0;
    let mut chem_total = 0;

    for other_chem in chem_list {
        let (other_amt, other_str) = other_chem;
        if other_str == "ORE" {
            
            break;
        }

        // wrong
        ore_total += other_amt + total_ore(map, other_str, *other_amt);

    }

    ore_total
}

fn part1(input: ChemMap) -> u32 {
    total_ore(&input, "FUEL", 1)
}

fn part2(_input: ChemMap) -> u32 {
    6969
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new_with_serializer(2019, 14, day14_gen);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
