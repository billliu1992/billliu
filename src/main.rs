mod blog;
mod error;
mod io;
mod template;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() {
    io::init_dirs(vec!["blog"]).unwrap();

    let (tx, rx) = mpsc::channel();
    let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();
    watcher.watch("input/", RecursiveMode::Recursive).unwrap();

    let start_time = Instant::now();

    out().unwrap();
    println!("T + 0s - Listening...");
    loop {
        match rx.recv().map(|_| out()) {
            Ok(_) => println!("T + {}s - Compiled!", start_time.elapsed().as_secs()),
            Err(e) => println!("Got error: {}", e),
        }
    }
}

fn out() -> error::EmptyResult {
    let mut templates = template::Templates::new();
    io::recursively_read_directory("input/templates", &mut |name, content| {
        templates.ingest(&name, &content)
    })?;

    let mut blogs = blog::BlogReader::new();
    io::recursively_read_directory("input/blog", &mut |_, content| blogs.ingest(content))?;

    let grass_options = grass::Options::default().style(grass::OutputStyle::Compressed);
    io::recursively_read_directory("input/css", &mut |name, content| -> error::EmptyResult {
        io::write_output_file(
            format!("{}.css", name),
            grass::from_string(content, &grass_options)?,
        )
    })?;

    io::write_output_file("index.html", templates.render_index(blogs.get_blogs())?)?;

    io::write_output_file(
        "blog-list.html",
        templates.render_blog_list(blogs.get_blogs())?,
    )?;

    for blog in blogs.get_blogs() {
        io::write_output_file(
            format!("blog/{}.html", blog.url_friendly_name),
            templates.render_blog(blog)?,
        )?;
    }

    Ok(())
}
