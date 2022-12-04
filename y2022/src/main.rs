use y2022::{day1, day2, day3, day4};

fn main() -> anyhow::Result<()> {
    println!("Advent of Code - 2022");
    day1::exec()?;
    day2::exec()?;
    day3::exec()?;
    day4::exec()?;

    Ok(())
}
