use std::{collections::HashMap, iter::zip};

use anyhow::{Context, Ok, Result};

fn get_two_lists(filename: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let contents = std::fs::read_to_string(filename)?;
    let split_lines = contents
        .lines()
        .map(|line| {
            line.split("   ")
                .map(|num| num.parse::<u32>().context("bad parse"))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;
    let (list1, list2) = split_lines
        .iter()
        .map(|number_list| (number_list[0], number_list[1]))
        .unzip();
    Ok((list1, list2))
}

// loglinear due to sorts
fn pz1_1(input: &str) -> Result<()> {
    let (mut list1, mut list2) = get_two_lists(input)?;
    list1.sort_unstable();
    list2.sort_unstable();

    let result: u32 = zip(list1, list2).map(|(a, b)| a.abs_diff(b)).sum();
    println!("{}", result);

    Ok(())
}

// amortized linear
fn pz1_2(input: &str) -> Result<()> {
    let (list1, list2) = get_two_lists(input)?;
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
    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    pz1_1("input/1.txt")?;
    pz1_2("input/1.txt")?;

    Ok(())
}
