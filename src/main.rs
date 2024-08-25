use afmt::set_context_and_format_code;
use anyhow::{bail, Context as AnyhowContext, Result};

fn main() -> Result<()> {
    let result = set_context_and_format_code().context("format_code() has `None` return.")?;

    println!("{}", result);
    Ok(())
}
