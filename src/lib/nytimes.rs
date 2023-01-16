use super::types::Article;
use reqwest::Url;

const URL: &str = "https://www.nytimes.com/section/opinion/editorials";

pub fn articles() -> Vec<Article> {
    let response = match reqwest::blocking::get(URL) {
        Ok(response) => response.text(),
        Err(_e) => return vec![],
    }
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("h2.css-1kv6qi").unwrap();
    let div_selector = scraper::Selector::parse(".css-1l4spti").unwrap();
    let a_selector = scraper::Selector::parse("a").unwrap();

    let titles = document
        .select(&title_selector)
        .take(5)
        .map(|x| x.inner_html());

    let urls = document
        .select(&div_selector)
        .take(5)
        .flat_map(|n| n.select(&a_selector))
        .filter_map(|x| x.value().attr("href"))
        .map(|x| format!("https://www.nytimes.com{}", x));

    titles
        .zip(urls)
        .map(|(title, link)| Article {
            title,
            link: Url::parse(&link).unwrap(),
        })
        .collect()
}
