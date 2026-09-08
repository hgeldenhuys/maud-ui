//! Regenerate pre-rendered API docs: cargo run --example build_docs.
//! Markdown parsing is a development dependency, never a consumer dependency.
use pulldown_cmark::{html::push_html, Options, Parser};
use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    for directory in ["docs/components", "docs/blocks"] {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.extension() != Some(std::ffi::OsStr::new("md"))
                || path.file_stem() == Some(std::ffi::OsStr::new("README"))
            {
                continue;
            }
            let markdown = fs::read_to_string(&path)?;
            let mut rendered = String::new();
            push_html(
                &mut rendered,
                Parser::new_ext(
                    &markdown,
                    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
                ),
            );
            let output = Path::new(directory).join("rendered");
            fs::create_dir_all(&output)?;
            fs::write(
                output
                    .join(path.file_name().unwrap())
                    .with_extension("html"),
                rendered,
            )?;
        }
    }
    println!("Rendered component and block API docs");
    Ok(())
}
