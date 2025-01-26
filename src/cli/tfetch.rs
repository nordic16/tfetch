use std::{io::Write, time::Duration};
use console::{Style, Term};
use indicatif::ProgressBar;
use reqwest::Client;
use crate::providers::{braflix::Braflix, Provider};

fn stdout() -> Term {
    Term::stdout()
}

pub async fn run() -> anyhow::Result<()> {
    let client : Client = Client::builder().referer(true).build()?;

    let term = stdout();
    term.clear_screen()?;
    let title = format!("{}", get_style().apply_to("Welcome to tfetch!\n"));

    term.hide_cursor()?;

    term.write_line(&title)?;
    write!(&term, "Search TV shows or anime: ")?; // I did NOT need that newline.
    
    let query = term.read_line()?;
    term.clear_screen()?;

    let bar = ProgressBar::new_spinner().with_message("Loading results");
    bar.enable_steady_tick(Duration::from_millis(100));

    let results = Braflix::fetch_results(&client, &query).await?;
    
    bar.finish();
    Ok(())
}

pub fn get_style() -> Style {
    Style::new().blink_fast().bold().blue()
}