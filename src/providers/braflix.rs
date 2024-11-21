use super::Provider;

#[derive(Default)]
pub struct Braflix {
    name: String,
    base_url: String,
}

impl Provider for Braflix {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn fetch_results(&self, query: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
