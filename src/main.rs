mod io;
mod error;

use comrak;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::error::Error;
use toml;
use toml::value::Datetime;

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

#[derive(Serialize, Deserialize)]
struct BlogRenderData {
    content: String,
    date: String,
    title: String,
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
    io::init_dirs(vec!("blog")).unwrap();

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
            href: format!("/blogs/{}.html", blog.metadata.url_friendly_name),
        });
    }

    let grass_options = grass::Options::default()
        .style(grass::OutputStyle::Compressed);
    io::recursively_read_directory("css", &mut |name, content| -> error::EmptyResult {
        io::write_output_file(
            format!("{}.css", name),
            grass::from_string(content, &grass_options)?)
    }).unwrap();

    io::write_output_file("index.html",
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
        io::write_output_file(
            format!("blogs/{}.html", blog.metadata.url_friendly_name),
            handlebars.render("blog", &BlogRenderData {
                content: blog.content,
                date: blog.metadata.date.to_string(),
                title: blog.metadata.title,
            }).unwrap(),
        )
        .unwrap();
    }
}

fn read_blogs() -> Result<Vec<Blog>, Box<dyn Error>> {
    let mut blogs: Vec<Blog> = Vec::new();
    io::recursively_read_directory("./blog", &mut |name, content| -> error::EmptyResult {
        let blog_divided: Vec<_> = content.splitn(3, "---").collect();
        if blog_divided.len() < 3 {
            return Err(Box::new(error::SiteError {
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

fn read_templates(handlebars: &mut Handlebars) -> error::EmptyResult {
    io::recursively_read_directory("./templates", &mut |name,
                                                               content|
     -> error::EmptyResult {
        handlebars.register_template_string(&name, content)?;
        Ok(())
    })
}