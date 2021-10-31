mod theme;

use std::error::Error;
use std::env;

extern crate dotenv;

use dotenv::dotenv;
use newsapi::{NewsAPI, Endpoint, Country, Article};

fn render_articles(articles: &[Article]) {
    let theme = theme::default();
    theme.print_inline("# Top headlines \n\n");
    for item in articles {
        theme.print_text(&format!("`{}`", item.title()));
        theme.print_text(&format!("> *{}*", item.url()));
        theme.print_text("---");
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY")?;

    let mut news_api = NewsAPI::new(&api_key);
    news_api.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    //
    // let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let news_api_response = news_api.fetch()?;

    render_articles(news_api_response.articles());
    Ok(())
}
