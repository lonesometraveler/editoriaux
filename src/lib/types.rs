use std::fmt::Display;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub link: url::Url,
}

impl<T> From<(String, T)> for Article
where
    T: AsRef<str> + Display,
{
    fn from((title, link): (String, T)) -> Self {
        Article {
            title,
            link: url::Url::parse(link.as_ref()).unwrap(),
        }
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {:?}", self.title, self.link.as_str())
    }
}

pub(crate) mod helper {
    pub fn get_html<T>(url: T) -> scraper::Html
    where
        T: reqwest::IntoUrl,
    {
        let response = match reqwest::blocking::get(url) {
            Ok(response) => response.text(),
            Err(_e) => return scraper::Html::parse_document(""),
        }
        .unwrap();
        scraper::Html::parse_document(&response)
    }
}
