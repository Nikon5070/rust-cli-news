mod theme;

use std::error::Error;
use std::env;

extern crate dotenv;

use dotenv::dotenv;
use lazy_static::lazy_static;
use newsapi::{get_articles, Articles};

lazy_static! {
    static ref API_KEY: String = env::var("API_KEY").unwrap();
}

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_inline("# Top headlines \n\n");
    for item in &articles.articles {
        theme.print_text(&format!("`{}`", item.title));
        theme.print_text(&format!("> *{}*", item.url));
        theme.print_text("---");
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = "https://newsapi.org/v2/everything?q=tesla&from=2021-09-30&sortBy=publishedAt&apiKey=";
    let url = format!("{}{}", url, *API_KEY);
    let articles = get_articles(&url)?;

    render_articles(&articles);
    Ok(())
}
