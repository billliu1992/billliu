use crate::error;
use comrak;
use serde::{Deserialize};
use toml;
use toml::value::Datetime;

pub struct Blog {
    pub title: String,
    pub descr: String,
    pub url_friendly_name: String,
    pub date: Datetime,
    pub content: String,
}

pub struct BlogReader {
    blogs: Vec<Blog>,
}

impl BlogReader {
    pub fn new() -> BlogReader {
        BlogReader {
            blogs: vec!()
        }
    }

    pub fn ingest(& mut self, blog: String) -> error::EmptyResult {
        let blog_divided: Vec<_> = blog.splitn(3, "---").collect();
        if blog_divided.len() < 3 {
            return Err(Box::new(error::SiteError {
                msg: format!(
                    "Blog: did not have 3 YAML section divided parts, had: {}",
                    blog_divided.len()
                ),
            }));
        }
        let metadata: BlogMetadata = toml::from_str(blog_divided[1])?;
        self.blogs.push(Blog {
            content: comrak::markdown_to_html(
                blog_divided[2],
                &comrak::ComrakOptions::default(),
            ),
            title: metadata.title,
            descr: metadata.descr,
            url_friendly_name: metadata.url_friendly_name,
            date: metadata.date,
        });
        return Ok(());
    }

    pub fn get_blogs(&self) -> &[Blog] {
        self.blogs.as_slice()
    }
}

#[derive(Deserialize)]
struct BlogMetadata {
    title: String,
    descr: String,
    url_friendly_name: String,
    date: Datetime,
}
