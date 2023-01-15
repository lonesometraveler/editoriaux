use url::Url;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub link: Url,
}
