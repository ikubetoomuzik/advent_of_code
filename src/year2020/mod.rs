//! crate for my answers
//! By: Curtis Jones <mail@curtisjones.ca>
//! Started Dec 1st, 2020

use std::io;

mod day_one;
mod day_two;

use day_one::day_one;
use day_two::day_two;

pub fn run() -> io::Result<()> {
    day_one()?;
    day_two()?;
    Ok(())
}
