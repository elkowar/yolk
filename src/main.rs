use anyhow::Result;

pub mod main_a;
// pub mod main_b;

pub fn main() -> Result<()> {
    main_a::main()?;
    Ok(())
}
