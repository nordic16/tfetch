use providers::Provider;

pub mod cli;
pub mod providers;

static PROVIDERS: Box<dyn Provider> = Box::<dyn Provider>::new();

pub(crate) fn main() -> anyhow::Result<()> {
    cli::tfetch::run()
}
