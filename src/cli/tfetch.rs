use std::io;

use crate::PROVIDERS;

pub fn run() -> anyhow::Result<()> {
    let mut buf = String::new();
    print!("Select provider");
    io::stdin().read_line(&mut buf)?;

    Ok(())
}
