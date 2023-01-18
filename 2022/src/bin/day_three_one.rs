#![feature(step_trait)]
use std::iter::Step;

const BASE: char = 'a';

fn char_to_priority(ch: char) -> usize {
    let val = Step::steps_between(&BASE, &ch.to_ascii_lowercase()).unwrap() + 1;
    if ch.is_uppercase() {
        val + 26
    } else {
        val
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("files/input_day3")?;
    let mut stick_outs = Vec::new();
    for line in file.lines() {
        let length = line.chars().count();
        let (first_sack, second_sack) = line.split_at(length / 2);
        for ch in first_sack.chars() {
            if second_sack.contains(ch) {
                // println!("Char: {}, Priority: {}", ch, char_to_priority(ch));
                stick_outs.push(char_to_priority(ch));
                break;
            }
        }
    }
    let total: usize = stick_outs.iter().sum();
    println!("Total priority is: {}", total);
    Ok(())
}
