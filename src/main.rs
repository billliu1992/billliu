use comrak;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use toml;
use toml::value::Datetime;

type EmptyResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
struct SiteError {
    msg: String,
}
impl Error for SiteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl Display for SiteError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

struct Blog {
    metadata: BlogMetadata,
    content: String,
}

#[derive(Deserialize)]
struct BlogMetadata {
    title: String,
    descr: String,
    url_friendly_name: String,
    date: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlogSummaryRenderData {
    title: String,
    descr: String,
    date_rendered: String,
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexTemplateRenderData {
    blog_summaries: Vec<BlogSummaryRenderData>,
}

fn main() {
    fs::create_dir_all("output/blogs").unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    read_templates(&mut handlebars).unwrap();

    let blogs = read_blogs().unwrap();

    let mut blog_summaries: Vec<BlogSummaryRenderData> = Vec::new();
    for blog in &blogs[0..=cmp::min(4, blog_summaries.len())] {
        blog_summaries.push(BlogSummaryRenderData {
            title: blog.metadata.title.clone(),
            descr: blog.metadata.descr.clone(),
            date_rendered: blog.metadata.date.to_string(),
            href: format!("/blogs/{}", blog.metadata.url_friendly_name),
        });
    }

    fs::write(
        "output/index.html",
        handlebars
            .render(
                "index",
                &IndexTemplateRenderData {
                    blog_summaries: blog_summaries,
                },
            )
            .unwrap(),
    )
    .unwrap();

    for blog in blogs {
        let mut data = BTreeMap::new();
        data.insert("content", blog.content);
        data.insert("date", blog.metadata.date.to_string());
        data.insert("title", blog.metadata.title);
        fs::write(
            format!("output/blogs/{}.html", blog.metadata.url_friendly_name),
            handlebars.render("blog", &data).unwrap(),
        )
        .unwrap();
    }
}

fn read_blogs() -> Result<Vec<Blog>, Box<dyn Error>> {
    let mut blogs: Vec<Blog> = Vec::new();
    recursively_read_directory(Path::new("./blog"), &mut |name, content| -> EmptyResult {
        let blog_divided: Vec<_> = content.splitn(3, "---").collect();
        if blog_divided.len() < 3 {
            return Err(Box::new(SiteError {
                msg: format!(
                    "Blog: {} did not have 4 YAML section divided parts, had: {}",
                    name,
                    blog_divided.len()
                ),
            }));
        }
        let metadata: BlogMetadata = toml::from_str(blog_divided[1])?;
        blogs.push(Blog {
            content: comrak::markdown_to_html(blog_divided[2], &comrak::ComrakOptions::default()),
            metadata,
        });
        return Ok(());
    })?;
    Ok(blogs)
}

fn read_templates(handlebars: &mut Handlebars) -> EmptyResult {
    recursively_read_directory(Path::new("./templates"), &mut |name,
                                                               content|
     -> EmptyResult {
        handlebars.register_template_string(&name, content)?;
        Ok(())
    })
}

fn recursively_read_directory(
    root_dir: &Path,
    handler: &mut dyn FnMut(String, String) -> EmptyResult,
) -> EmptyResult {
    let mut view_dir_files = fs::read_dir(root_dir)?;
    while let Some(dir_entry) = view_dir_files.next() {
        let path = dir_entry?.path();
        if path.is_dir() {
            recursively_read_directory(path.as_path(), handler)?;
        } else {
            let file_stem = path
                .file_stem()
                .map(|f| f.to_str())
                .flatten()
                .map(|s| s.to_string());

            if let Some(stem) = file_stem {
                handler(stem, fs::read_to_string(path)?)?;
            }
        }
    }
    Ok(())
}
