use std::{fs, path::PathBuf, collections::HashMap};
use regex::Regex;

// handle the memory management of an article
#[derive(Clone, Debug)]
pub struct Article {
    pub author: String,
    pub date: String,
    pub title: String,
    pub language: String,
    pub md_content: String,  // markdown article
    pub description: String,
    pub illustration_table_html: String,  // HTML of the illustation table
    pub cached_html: Option<String>, // cache the html version to avoid parsing it at each call
}

lazy_static! {
    static ref IMAGE_LINK_REGEX: Regex = Regex::new(r"\!\[(.*)\]\((.*)\)").unwrap();
}

impl Article {
    pub fn new(file_path: &PathBuf, should_cache: bool) -> Self {
        let md = fs::read_to_string(&file_path).unwrap();  
        let mut header_separator_count = 0;
        let mut image_links: HashMap<String, String> = HashMap::new();  // Auto sources the images links 

        // Default values
        let mut author = "Unknown author".into();
        let mut date = "Unknown date".into();
        let mut title = "Unknown title".into();
        let mut language = "fr".to_uppercase().into();
        let mut description = "".into();
        let mut illustration_table_html: String = "".into();

         // Auto sources the images links 
        for (_, [desc, link]) in IMAGE_LINK_REGEX.captures_iter(&md).map(|c| c.extract()) {
            image_links.insert(desc.into(), link.into());
        }

        // parse markdown
        for line in md.lines() {
            if line.contains("---") {
                header_separator_count += 1;
                continue;
            }
            if header_separator_count < 2 {
                // Parse metadata        
                if let Some((_key, _val)) = line.split_once(":") {
                    let key = String::from(_key);
                    let val = String::from(_val);
                    match key.to_lowercase().trim() {
                        "author" => { author = val },
                        "date" => { date = val },
                        "title" => { title = val },
                        "language" => { language = val.to_uppercase() },
                        "description" => { description = val },
                        _ => {}
                    }
                }
                continue;
            } 
            // Parse the body
            if description.is_empty() && line.get(0..1).unwrap_or("") == ">"  {
                // if no description it take the first quote
                description = line[1..].into();
            }
        }

        // Add the images links at the end of the markdown document
        if image_links.len() > 0 {
            illustration_table_html += "\n <h2>Illustations</h2>";
            illustration_table_html += "\n <ol>";
            image_links.iter().for_each(|(name, link)| {
                illustration_table_html += format!("\n<li><p><a href={}>{}</a></p></li>", link, name).as_str();
            });
        }
        

        let cached_html = if should_cache { 
            Some(Self::markdown_to_html(&md) + &illustration_table_html)
        } else { 
            None
        };

        Self { 
            author, date, title, language, 
            description,
            md_content: md,
            illustration_table_html, 
            cached_html,
        }
    }

    pub fn get_content(&self, get_raw: bool) -> String {
        if get_raw { 
            self.md_content.clone()
        } else { 
            self.cached_html.clone().unwrap_or(
                Self::markdown_to_html(&self.md_content) + &self.illustration_table_html
            )
            
        }
    }

    pub fn markdown_to_html(md: &str) -> String {
        let constructs = markdown::Constructs {
            math_flow: false,
            math_text: false,
            frontmatter: true,  // Metadata Header
            gfm_footnote_definition: true,
            gfm_label_start_footnote: true,
            ..markdown::Constructs::gfm()
        };
        let parse = markdown::ParseOptions { 
            constructs, 
            math_text_single_dollar: true,
            ..markdown::ParseOptions::gfm()
        };
        let options = markdown::Options {
            parse,
            ..markdown::Options::gfm()
        };
        markdown::to_html_with_options(&md, &options).unwrap_or("".into())
    }
}