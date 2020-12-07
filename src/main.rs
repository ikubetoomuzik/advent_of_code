//! crate for my answers
//! By: Curtis Jones <mail@curtisjones.ca>
//! Started Dec 1st, 2020

use std::io;

mod year2020;

fn main() -> io::Result<()> {
    year2020::run()?;
    Ok(())
}
