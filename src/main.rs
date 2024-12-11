pub mod input;
use std::{
    collections::HashMap,
    iter::{once, zip},
};

use anyhow::{Ok, Result};
use itertools::Itertools;

// loglinear due to sorts
fn pz1_1(input: &str) -> Result<u32> {
    let (mut list1, mut list2) = input::two_lists(input)?;
    list1.sort_unstable();
    list2.sort_unstable();

    let result: u32 = zip(list1, list2).map(|(a, b)| a.abs_diff(b)).sum();

    Ok(result)
}

// amortized linear
fn pz1_2(input: &str) -> Result<u32> {
    let (list1, list2) = input::two_lists(input)?;
    let multiplicities: HashMap<u32, u32> =
        list2
            .iter()
            .fold(HashMap::with_capacity(list2.len()), |mut acc, &num| {
                acc.entry(num).and_modify(|mult| *mult += 1).or_insert(1);
                acc
            });

    let result: u32 = list1
        .iter()
        .map(|num| multiplicities.get(num).map_or(0, |mult| num * mult))
        .sum();

    Ok(result)
}

// linear
fn pz2_1(input: &str) -> Result<usize> {
    let mut list = input::list_per_line(input)?;

    // a little strange; if first two digits are ascending,
    // then reverse it, otherwise don't touch it; then we can
    // treat the cases uniformly
    for line in list.iter_mut() {
        if line[0] < line[1] {
            line.reverse();
        }
    }

    let result: usize = list
        .iter()
        .map(|line| {
            line.windows(2)
                .map(|pair| pair[0] - pair[1])
                .all(|diff| diff >= 1 && diff <= 3)
        })
        .filter(|&b| b)
        .count();

    Ok(result)
}

// quadratic
// n.b.; a linear solution is very much possible, although a little painful;
// essentially a list can permit up to two differences < 1 or > 3 next to each other
// somewhere in the list, and the bad apple is one of the two elements in the lattermost
// discrepant pair. so there's only two places to check for bad-apple-ness. but on
// this data it's totally not worth it; quadratic is likely faster, even
fn pz2_2(input: &str) -> Result<usize> {
    let list = input::list_per_line(input)?;

    fn is_valid(line: &[&u32]) -> bool {
        line.windows(2)
            .map(|pair| pair[0] - pair[1])
            .all(|diff| diff >= 1 && diff <= 3)
            || line
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .all(|diff| diff >= 1 && diff <= 3)
    }

    let result: usize = list
        .iter()
        .map(|line| {
            let full_line_ref = line.iter().collect::<Vec<&u32>>();
            line.iter()
                .combinations(line.len() - 1) // all but one
                .chain(once(full_line_ref)) // plus the all case
                .any(|comb| is_valid(&comb))
        })
        .filter(|&b| b)
        .count();

    Ok(result)
}

fn main() -> Result<()> {
    println!("day 1, part 1: {}", pz1_1("input/1.txt")?);
    println!("day 1, part 2: {}", pz1_2("input/1.txt")?);
    println!();

    println!("day 2, part 1: {}", pz2_1("input/2.txt")?);
    println!("day 2, part 2: {}", pz2_2("input/2.txt")?);
    println!();

    Ok(())
}
