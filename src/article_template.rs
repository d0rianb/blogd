use askama::Template;

use crate::article::Article;

// handle the rendering aspect of an article
#[derive(Template, Clone, Debug)]
#[template(path = "../static/article.html")]
pub struct ArticleTemplate<'a> {
    article: &'a Article,
    html_content: String
}

impl<'a> ArticleTemplate<'a> {
    pub fn new(article: &'a Article, get_raw: bool) -> Self { 
        Self { article, html_content: article.get_content(get_raw) } 
    }
}