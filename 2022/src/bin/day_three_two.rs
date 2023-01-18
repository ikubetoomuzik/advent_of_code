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

fn get_badge(sacks: &[String]) -> Option<usize> {
    for ch in sacks[0].chars() {
        if sacks[1].contains(ch) && sacks[2].contains(ch) {
            return Some(char_to_priority(ch));
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("files/input_day3")?;
    let mut three_lines = [String::new(), String::new(), String::new()];
    let mut badges = 0;
    for (idx, line) in file.lines().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            badges += get_badge(three_lines.as_ref()).unwrap();
        }
        three_lines[idx % 3] = String::from(line);
    }
    badges += get_badge(three_lines.as_ref()).unwrap();
    println!("Total priority is: {}", badges);
    Ok(())
}
