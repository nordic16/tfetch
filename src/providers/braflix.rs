use std::rc::Rc;

use reqwest::Client;
use soup::{NodeExt, QueryBuilderExt, Soup};

use super::Provider;

#[derive(Default)]
pub struct Braflix;

impl Provider for Braflix {
    fn get_name<'a>() -> &'a str {
        "Braflix"
    }

    fn get_base_url<'a>() -> &'a str {
        "https://braflix.club"
    }

    async fn fetch_results(client: &Client,query: &str) -> anyhow::Result<Vec<String>> {
        let search_url = format!("{}/search/{}", Braflix::get_base_url(), query);
        let req = client.get(&search_url).send().await?;
        
        let html = Soup::new(&req.text().await?);
        let shows: Vec<_> = html.tag("a").class("aspect-poster").find_all().collect();
        let urls: Vec<String> = shows.iter().map(|s| s.get("href").unwrap_or_default()).collect();

        Ok(urls)
    }
}
