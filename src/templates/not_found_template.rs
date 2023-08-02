use askama::Template;

#[derive(Template, Clone, Debug)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;