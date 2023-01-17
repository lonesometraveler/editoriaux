use std::fmt::Display;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub link: url::Url,
}

impl<T, U> From<(T, U)> for Article
where
    T: Display,
    U: AsRef<str> + Display,
{
    fn from((title, link): (T, U)) -> Self {
        Article {
            title: title.to_string(),
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
