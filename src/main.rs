pub mod cli;
pub mod providers;
pub mod tests;

pub(crate) fn main() -> anyhow::Result<()> {
    cli::tfetch::run()
}
