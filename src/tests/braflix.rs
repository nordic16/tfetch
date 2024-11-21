#[cfg(test)]
mod tests {
    use crate::providers::{braflix::Braflix, Provider};

    fn fetch_results() -> anyhow::Result<()> {
        let query = "arcane";
        let prov = Braflix::default();
        prov.fetch_results(&query)?;
        Ok(())
    }
}
