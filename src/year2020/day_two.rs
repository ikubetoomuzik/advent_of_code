//! Module for the day two problems.
//! By Curtis Jones <mail@curtisjones.ca>
//!
//! Filtering a list of passwords defined as format:
//! $num-$num $char: $password
//!
//! The first 2 nums switch meaning but the $char is always something
//! we are looking for in the $password.

use std::{
    fs::OpenOptions,
    io::{self, Read},
    time::Instant,
};

fn day_two_puzzle_two(passwords: &str) -> usize {
    passwords
        .lines()
        .filter(|pw| {
            let mut pw = pw.split(' ');
            let (pos1, pos2) = {
                let mut tmp = pw.next().unwrap().split('-');
                (
                    tmp.next().unwrap().parse::<usize>().unwrap(),
                    tmp.next().unwrap().parse::<usize>().unwrap(),
                )
            };
            let chr = pw.next().unwrap().chars().next().unwrap();
            let pw = pw.next().unwrap();
            // XOR -- only one can be true
            (pw.chars().nth(pos1 - 1).unwrap() == chr) ^ (pw.chars().nth(pos2 - 1).unwrap() == chr)
        })
        .count()
}

fn day_two_puzzle_one(passwords: &str) -> usize {
    passwords
        .lines()
        .filter(|pw| {
            let mut pw = pw.split(' ');
            let (min, max) = {
                let mut tmp = pw.next().unwrap().split('-');
                (
                    tmp.next().unwrap().parse::<usize>().unwrap(),
                    tmp.next().unwrap().parse::<usize>().unwrap(),
                )
            };
            let chr = pw.next().unwrap().chars().next().unwrap();
            let pw = pw.next().unwrap();
            let matches = pw.matches(chr).count();
            matches >= min && matches <= max
        })
        .count()
}

pub fn day_two() -> io::Result<()> {
    // load input
    let mut input = String::new();
    OpenOptions::new()
        .read(true)
        .open("inputs/2020D2")?
        .read_to_string(&mut input)?;
    let lines = input.lines().count();
    //puzzle one
    let t1 = Instant::now();
    let res = day_two_puzzle_one(&input);
    let t2 = Instant::now();
    println!(
        "Puzzle One:\n\
    There are {}/{} valid passwords!\n\
    Calculated in {}ms.",
        res,
        lines,
        (t2 - t1).as_millis(),
    );
    //puzzle two
    let t1 = Instant::now();
    let res = day_two_puzzle_two(&input);
    let t2 = Instant::now();
    println!(
        "Puzzle Two:\n\
    There are {}/{} valid passwords!\n\
    Calculated in {}ms.",
        res,
        lines,
        (t2 - t1).as_millis(),
    );
    Ok(())
}
