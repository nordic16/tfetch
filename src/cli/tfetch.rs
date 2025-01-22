use std::{io::{self, Write}, sync::mpsc::channel};

use console::{Style, Term};
use reqwest::{Client, ClientBuilder};

use crate::{cli::TMessage, providers::{self, braflix::Braflix, Provider}};

fn stdout() -> Term {
    Term::stdout()
}


pub async fn run() -> anyhow::Result<()> {
    let client : Client = Client::builder().referer(true).build()?;

    print!("{}", "\x1B[2J\x1B[1;1H"); // clears term and sets cursor pos to (1,1)
    let term = stdout();
    let title = format!("{}", get_style().apply_to("Welcome to tfetch!\n"));



    term.hide_cursor()?;

    term.write_line(&title)?;
    write!(&term, "Search TV shows or anime: ")?; // I did NOT need that newline.
    
    let query = term.read_line()?;
    let results = Braflix::fetch_results(&client, &query).await?;
    
    Ok(())
}

pub fn get_style() -> Style {
    Style::new().blink_fast().bold().blue()
}