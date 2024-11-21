pub mod braflix;

pub trait Provider: Default {
    fn get_name(&self) -> &str;
    fn get_base_url(&self) -> &str;
    fn fetch_results(&self, query: &str) -> anyhow::Result<()>;
}
