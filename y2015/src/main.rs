use std::error::Error;
use y2015::{day1, day2};

fn main() -> Result<(), Box<dyn Error>> {
    day1::execute()?;
    day2::execute()?;

    Ok(())
}