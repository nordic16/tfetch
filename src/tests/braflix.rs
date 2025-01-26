

#[tokio::test]
async fn fetch_results() -> anyhow::Result<()> {
    use reqwest::Client;
    use crate::providers::{braflix::Braflix, Provider};

    let client = Client::new();
    let query = "one piece";
    let res = Braflix::fetch_results(&client, &query).await?;

    res.iter().for_each(|f| println!("{}", f));

    Ok(())
}
