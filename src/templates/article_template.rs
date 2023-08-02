use askama::Template;

use crate::article::Article;

// handle the rendering aspect of an article
#[derive(Template, Clone, Debug)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    article: &'a Article,
    html_content: String
}

impl<'a> ArticleTemplate<'a> {
    pub fn new(article: &'a Article) -> Self { 
        Self { article, html_content: article.get_content(false) } 
    }
}

// handle the rendering aspect of an raw article (md version)
#[derive(Template, Clone, Debug)]
#[template(path = "raw_article.html")]
pub struct RawArticleTemplate<'a> {
    article: &'a Article,
    html_content: String
}

impl<'a> RawArticleTemplate<'a> {
    pub fn new(article: &'a Article) -> Self { 
        Self { article, html_content: article.get_content(true) } 
    }
}