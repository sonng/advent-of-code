use anyhow::Result;
use y2015::{day1, day2, day3, day4, day5, day6};

fn main() -> Result<()> {
    day1::execute()?;
    day2::execute()?;
    day3::execute()?;
    // day4::execute()?;
    day5::execute()?;
    day6::execute()?;

    Ok(())
}
