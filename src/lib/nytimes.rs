use super::types::Article;

const URL: &str = "https://www.nytimes.com/section/opinion/editorials";

pub fn articles() -> Vec<Article> {
    let document = super::types::helper::get_html(URL);

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

    titles.zip(urls).map(|x| x.into()).collect()
}
