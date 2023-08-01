mod article;
mod article_template;

use std::fs;
use actix_files;

use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpServer, Result, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_files::NamedFile;
use article::Article;
use askama::Template;

use std::collections::HashMap;
use std::time::Instant;

use crate::article_template::ArticleTemplate;


const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DEBUG : bool = true;

#[derive(Clone)]
struct AppData {
    articles: HashMap<String, Article>,
}

async fn get_index(_req: HttpRequest) -> Result<NamedFile> {
    let index: NamedFile = NamedFile::open("../static/index.html")?;
    Ok(index)
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
            let template = ArticleTemplate::new(&article, get_raw_file);
            let body = template.render().unwrap();
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            Ok(
                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(body)
            )
         },
        None => not_found(req).await
    }
}

async fn not_found(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/404.html"))
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

fn reload_articles_data() -> AppData {
    let mut app_data = AppData { articles: HashMap::new() };
    init_data(&mut app_data);
    app_data
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
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
        .bind((ADDRESS, PORT))?
        .run()
        .await
}