use crate::error;
use chrono::naive::NaiveDate;
use comrak;
use serde::Deserialize;
use std::str::FromStr;
use toml;
use toml::value::Datetime;

pub struct Blog {
    pub title: String,
    pub descr: String,
    pub url_friendly_name: String,
    pub date: NaiveDate,
    pub content: String,
}

pub struct BlogReader {
    blogs: Vec<Blog>,
}

impl BlogReader {
    pub fn new() -> BlogReader {
        BlogReader { blogs: vec![] }
    }

    pub fn ingest(&mut self, blog: String) -> error::EmptyResult {
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
            content: comrak::markdown_to_html(blog_divided[2], &comrak::ComrakOptions::default()),
            title: metadata.title,
            descr: metadata.descr,
            url_friendly_name: metadata.url_friendly_name,
            date: NaiveDate::from_str(&metadata.date.to_string())?,
        });

        self.blogs.sort_by(|a, b| b.date.cmp(&a.date));
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ingest() -> error::EmptyResult {
        let mut subject = BlogReader::new();

        subject.ingest(String::from(
"---
title = \"Test Title\"
descr = \"Test description.\"
url_friendly_name = \"url-friendly\"
date = 2020-01-01
---
This is a test

With Markdown
"))?;

        assert!(subject.get_blogs().len() == 1);
        assert!(subject.get_blogs()[0].title == "Test Title");
        assert!(subject.get_blogs()[0].descr == "Test description.");
        assert!(subject.get_blogs()[0].url_friendly_name == "url-friendly");
        // This assertion is highly dependent on the Markdown library.
        // It's easier to get the output first, then set it as the correct value.
        assert!(subject.get_blogs()[0].content ==
"<p>This is a test</p>
<p>With Markdown</p>
");
        
        Ok(())
    }

    #[test]
    fn test_get_blogs_correct_order() -> error::EmptyResult {
        let mut subject = BlogReader::new();

        subject.ingest(String::from(
"---
title = \"Test 1\"
descr = \"Test.\"
url_friendly_name = \"url-friendly\"
date = 2020-01-01
---
This is a test

With Markdown
"))?;
        subject.ingest(String::from(
"---
title = \"Test 2\"
descr = \"Test.\"
url_friendly_name = \"url-friendly\"
date = 2020-01-03
---
This is a test

With Markdown
"))?;
        subject.ingest(String::from(
"---
title = \"Test 3\"
descr = \"Test.\"
url_friendly_name = \"url-friendly\"
date = 2020-01-02
---
This is a test

With Markdown
"))?;
        
        assert!(subject.get_blogs().len() == 3);
        assert!(subject.get_blogs()[0].title == "Test 2");
        assert!(subject.get_blogs()[1].title == "Test 3");
        assert!(subject.get_blogs()[2].title == "Test 1");

        Ok(())
    }
}