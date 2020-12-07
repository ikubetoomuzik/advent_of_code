//! Module for just the day one problems.
//! By: Curtis Jones <mail@curtisjones.ca>

use std::{
    fs::OpenOptions,
    io::{self, Read},
    time::Instant,
};

fn day_one_puzzle_one_alg(start_idx: usize, comp_idx: usize, list: &[i32]) -> (usize, usize, i32) {
    match list[start_idx] + list[comp_idx] {
        2020 => (start_idx, comp_idx, list[start_idx] * list[comp_idx]),
        _ => {
            if comp_idx == list.len() - 1 {
                assert_ne!(start_idx, list.len() - 1);
                day_one_puzzle_one_alg(start_idx + 1, start_idx + 2, list)
            } else {
                day_one_puzzle_one_alg(start_idx, comp_idx + 1, list)
            }
        }
    }
}

fn day_one_puzzle_two_alg(list: &[i32]) -> (usize, usize, usize, i32) {
    for (i, val_1) in list.iter().enumerate() {
        // get the diff we are looking to make up with the next vals.
        let diff = 2020 - val_1;
        for (j, val_2) in list.iter().enumerate() {
            // if we are already over the diff then no point in doing this iteration.
            if *val_2 >= diff {
                continue;
            }
            for (k, val_3) in list.iter().enumerate() {
                if val_2 + val_3 == diff {
                    // short circuit when we have our value.
                    return (i, j, k, val_1 * val_2 * val_3);
                }
            }
        }
    }
    unreachable!();
}

pub fn day_one() -> io::Result<()> {
    // load input
    let mut input = String::new();
    OpenOptions::new()
        .read(true)
        .open("inputs/2020D1")?
        .read_to_string(&mut input)?;
    let input: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let t1 = Instant::now();
    let res = day_one_puzzle_one_alg(0, 1, &input);
    let t2 = Instant::now();
    println!(
        "Puzzle One:\n\
    The first number is at index {} with value: {}.\n\
    The second number is at index {} with value: {}.\n\
    The answer is {}. And all this took: {}ms.",
        res.0,
        input[res.0],
        res.1,
        input[res.1],
        res.2,
        (t2 - t1).as_millis()
    );
    let t1 = Instant::now();
    let res = day_one_puzzle_two_alg(&input);
    let t2 = Instant::now();
    println!(
        "Puzzle Two:\n\
    The first number is at index {} with value: {}.\n\
    The second number is at index {} with value: {}.\n\
    The third number is at index {} with value: {}.\n\
    The answer is {}. And all this took: {}ms.",
        res.0,
        input[res.0],
        res.1,
        input[res.1],
        res.2,
        input[res.2],
        res.3,
        (t2 - t1).as_millis()
    );
    Ok(())
}
