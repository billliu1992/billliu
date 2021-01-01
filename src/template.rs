use handlebars::Handlebars;
use crate::error;
use crate::blog::Blog;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::cmp;

pub struct Templates<'a> {
    handlebars: Handlebars<'a>,
}

impl Templates<'_> {
    pub fn new<'a>() -> Templates<'a> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        handlebars.register_template_string("link-about", "/about.html").unwrap();
        handlebars.register_template_string("link-resume", "/static/resume.pdf").unwrap();
        handlebars.register_template_string("link-blog", "/blog-list.html").unwrap();
        handlebars.register_template_string("link-linkedin", "https://www.linkedin.com/in/billliu1992/").unwrap();
        handlebars.register_template_string("link-github", "https://github.com/billliu1992").unwrap();
        handlebars.register_template_string("link-twitter", "https://twitter.com/bill_liu_il/").unwrap();

        Templates { handlebars }
    }

    pub fn ingest(&mut self, title: &str, content: &str) -> error::EmptyResult {
        self.handlebars.register_template_string(title, content)?;
        Ok(())
    }

    pub fn render_index(&self, blogs: &[Blog]) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars
            .render(
                "index",
                &IndexTemplateRenderData {
                    blog_summaries: Templates::blogs_to_summary_render_datas(
                        blogs.get(0..=cmp::min(4, blogs.len() - 1)).ok_or("No blog summaries")?
                    )
                },
            )?)
    }

    pub fn render_blog_list(&self, blogs: &[Blog]) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars
            .render(
                "blog-list",
                &BlogListRenderData {
                    blog_summaries: Templates::blogs_to_summary_render_datas(blogs),
                },
            )?)
    }

    pub fn render_blog(&self, blog: &Blog) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars
            .render(
                "blog",
                &BlogRenderData {
                    content: &blog.content,
                    date: &blog.date.to_string(),
                    descr: &blog.descr,
                    title: &blog.title,
                },
            )?)
    }

    fn blogs_to_summary_render_datas(blogs: &[Blog]) -> Vec<BlogSummaryRenderData> {
        let mut blog_summaries: Vec<BlogSummaryRenderData> = Vec::new();
        for blog in blogs {
            blog_summaries.push(BlogSummaryRenderData {
                title: &blog.title,
                descr: &blog.descr,
                date_rendered: blog.date.to_string(),
                href: format!("/blog/{}.html", blog.url_friendly_name),
            });
        }
        blog_summaries
    }
}

#[derive(Serialize, Deserialize)]
struct BlogRenderData<'a> {
    content: &'a str,
    descr: &'a str,
    date: &'a str,
    title: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlogSummaryRenderData<'a> {
    title: &'a str,
    descr: &'a str,
    date_rendered: String,
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexTemplateRenderData<'a> {
    #[serde(borrow)]
    blog_summaries: Vec<BlogSummaryRenderData<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlogListRenderData<'a> {
    #[serde(borrow)]
    blog_summaries: Vec<BlogSummaryRenderData<'a>>,
}