use std::error::Error;
use std::env;

extern crate dotenv;

use dotenv::dotenv;
use colour::{dark_green, dark_blue};
use lazy_static::lazy_static;
use newsapi::{get_articles, Articles};

lazy_static! {
    static ref API_KEY: String = env::var("API_KEY").unwrap();
}

fn render_articles(articles: &Articles) {
    for item in &articles.articles {
        dark_green!("> {}\n", item.title);
        dark_blue!("> {}\n\n", item.url);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = &format!("https://newsapi.org/v2/everything?q=tesla&from=2021-09-30&sortBy=publishedAt&apiKey={}", *API_KEY);
    let articles = get_articles(url)?;

    render_articles(&articles);
    Ok(())
}
