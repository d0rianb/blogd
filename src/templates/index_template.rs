use std::collections::HashMap;

use askama::Template;

use crate::article::Article;

#[derive(Template, Clone, Debug)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub articles: &'a HashMap<String, Article>
}