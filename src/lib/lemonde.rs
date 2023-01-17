use super::types::Article;

const URL: &str = "https://www.lemonde.fr/editoriaux/";

pub fn articles() -> Vec<Article> {
    let document = super::types::helper::get_html(URL);

    let title_selector = scraper::Selector::parse("h3.teaser__title").unwrap();
    let link_selector = scraper::Selector::parse("a.teaser__link").unwrap();

    let titles = document
        .select(&title_selector)
        .take(5)
        .map(|x| x.inner_html());

    let urls = document
        .select(&link_selector)
        .take(5)
        .filter_map(|x| x.value().attr("href"));

    titles.zip(urls).map(|x| x.into()).collect()
}
