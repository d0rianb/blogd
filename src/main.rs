mod article;
mod templates;

use std::fs;
use actix_files;

use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpServer, Result, HttpResponse, Responder};
use actix_web::http::StatusCode;
use article::Article;
use templates::not_found_template::NotFoundTemplate;

use std::collections::HashMap;
use std::time::Instant;

use askama::Template;

use crate::templates::article_template::{ArticleTemplate, RawArticleTemplate};
use crate::templates::index_template::IndexTemplate;

const DEBUG : bool = true;

#[derive(Clone)]
struct AppData {
    pub articles: HashMap<String, Article>,
}

#[inline]
fn html_response(body: String) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(body)
    )
}

async fn get_index(_req: HttpRequest, app_data: web::Data<AppData>,) -> Result<impl Responder> {
    let template = IndexTemplate { articles: &app_data.articles };
    let body = template.render().unwrap();
    html_response(body)
}

async fn get_article(req: HttpRequest, app_data: web::Data<AppData>, info: web::Path<(String, )>) -> Result<impl Responder> {
    let article_id = &info.0;
    let get_raw_file = article_id.contains(".md");
    let normalized_id = article_id
        .to_lowercase()
        .replace(".md", "");
    match app_data.articles.get(&normalized_id) {
        Some(article) => { 
            let now = Instant::now();
            let body = if get_raw_file { 
                RawArticleTemplate::new(&article).render().unwrap()
            } else { 
                ArticleTemplate::new(&article).render().unwrap()
            };
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            html_response(body)
         },
        None => not_found(req).await
    }
}

async fn not_found(_req: HttpRequest) -> Result<HttpResponse> {
    let template = NotFoundTemplate { };
    let body = template.render().unwrap();
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(body)
    )
}

fn init_data(app_data: &mut AppData) {
    let articles_path: &str = "./articles";
    for entry in fs::read_dir(articles_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        // Filter only the md files
        if let Some(ext) = file_path.extension() { 
            if ext != "md" { continue; } 
        } else {
            continue;
        }
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_lowercase()
            .replace(".md", "");
        let article = Article::new(&file_path, !DEBUG);
        app_data.articles.insert(file_name, article);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    if DEBUG { std::env::set_var("RUST_LOG", "debug") };
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse::<u16>()
        .unwrap_or(8080);
    let address: String = std::env::var("ADDRESS").unwrap_or("127.0.0.1".into());
    HttpServer::new(move || {
        let mut app_data = AppData { articles: HashMap::new() };
        init_data(&mut app_data);
        App::new()
            .app_data(Data::new(app_data))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(actix_files::Files::new("/static", "./static").index_file("index.html"))
            .route("/", web::get().to(get_index))
            .route("articles/{article_id}", web::get().to(get_article))
            .default_service(web::route().to(not_found))
    })
        .bind((address, port))?
        .run()
        .await
}