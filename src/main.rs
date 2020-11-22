mod error;
mod io;

use comrak;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::error::Error;
use toml;
use toml::value::Datetime;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc;
use std::time::{Instant, Duration};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
struct BlogListRenderData {
    blog_summaries: Vec<BlogSummaryRenderData>,
}

fn main() {
    io::init_dirs(vec!["blog"]).unwrap();

    let (tx, rx) = mpsc::channel();
    let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();
    watcher.watch("input/", RecursiveMode::Recursive).unwrap();

    let start_time = Instant::now();

    println!("T + 0s - Listening...");
    loop {
        match rx.recv().map(|_| out()) {
            Ok(_) => println!("T + {}s - Compiled!", start_time.elapsed().as_secs()),
            Err(e) => println!("Got error: {}", e),
        }
    }
    
}

fn out() -> error::EmptyResult {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    read_templates(&mut handlebars)?;

    let blogs = read_blogs()?;

    let mut blog_summaries: Vec<BlogSummaryRenderData> = Vec::new();
    for blog in &blogs {
        blog_summaries.push(BlogSummaryRenderData {
            title: blog.metadata.title.clone(),
            descr: blog.metadata.descr.clone(),
            date_rendered: blog.metadata.date.to_string(),
            href: format!("/blogs/{}.html", blog.metadata.url_friendly_name),
        });
    }

    let grass_options = grass::Options::default().style(grass::OutputStyle::Compressed);
    io::recursively_read_directory("input/css", &mut |name, content| -> error::EmptyResult {
        io::write_output_file(
            format!("{}.css", name),
            grass::from_string(content, &grass_options)?,
        )
    })?;

    io::write_output_file(
        "index.html",
        handlebars
            .render(
                "index",
                &IndexTemplateRenderData {
                    blog_summaries: blog_summaries
                        .get(0..=cmp::min(4, blog_summaries.len() - 1))
                        .ok_or("No blog summaries")?
                        .to_vec(),
                },
            )?,
    )?;

    io::write_output_file(
        "blog-list.html",
        handlebars
            .render(
                "blog-list",
                &IndexTemplateRenderData {
                    blog_summaries: blog_summaries,
                },
            )?,
    )?;

    for blog in blogs {
        io::write_output_file(
            format!("blogs/{}.html", blog.metadata.url_friendly_name),
            handlebars
                .render(
                    "blog",
                    &BlogRenderData {
                        content: blog.content,
                        date: blog.metadata.date.to_string(),
                        title: blog.metadata.title,
                    },
                )?,
        )?;
    }

    Ok(())
}

fn read_blogs() -> Result<Vec<Blog>, Box<dyn Error>> {
    let mut blogs: Vec<Blog> = Vec::new();
    io::recursively_read_directory("input/blog", &mut |name, content| -> error::EmptyResult {
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
    io::recursively_read_directory("input/templates", &mut |name, content| -> error::EmptyResult {
        handlebars.register_template_string(&name, content)?;
        Ok(())
    })
}
