use afmt::format_code;
use anyhow::{bail, Context as AnyhowContext, Result};

fn main() -> Result<()> {
    let result = format_code().context("format_code() has `None` return.")?;

    println!("{}", result);
    Ok(())
}
