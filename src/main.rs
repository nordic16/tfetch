pub mod cli;
pub mod providers;
pub mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::tfetch::run().await
}
