use reqwest::Client;

pub mod braflix;

pub trait Provider: Default {
    fn get_name<'a>() -> &'a str;
    fn get_base_url<'a>() -> &'a str;
    async fn fetch_results(client: &Client, query: &str) -> anyhow::Result<Vec<String>>;
}
