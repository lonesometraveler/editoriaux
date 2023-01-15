use super::types::Article;
use reqwest::{IntoUrl, Url};

const URL: &str = "https://www.lemonde.fr/editoriaux/";

pub fn articles() -> Vec<Article> {
    get_articles(URL, "h3.teaser__title", "a.teaser__link")
}

fn get_articles<T>(url: T, title_selector: &str, link_selector: &str) -> Vec<Article>
where
    T: IntoUrl,
{
    let response = match reqwest::blocking::get(url) {
        Ok(response) => response.text(),
        Err(_e) => return vec![],
    }
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse(title_selector).unwrap();
    let link_selector = scraper::Selector::parse(link_selector).unwrap();
    document
        .select(&title_selector)
        .take(5)
        .map(|x| x.inner_html().replace("&nbsp;", ""))
        .zip(
            document
                .select(&link_selector)
                .filter_map(|x| x.value().attr("href")),
        )
        .map(|(title, link)| Article {
            title,
            link: Url::parse(link).unwrap(),
        })
        .collect()
}
