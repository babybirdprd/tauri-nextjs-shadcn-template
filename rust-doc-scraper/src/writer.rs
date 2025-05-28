use rayon::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

pub fn write_markdown_batch(pages: Vec<(String, String)>, out_dir: &str) -> anyhow::Result<()> {
    create_dir_all(out_dir)?;

    pages.into_par_iter().for_each(|(slug, content)| {
        let path = Path::new(out_dir).join(format!("{}.md", slug));
        let _ = File::create(&path).and_then(|mut f| f.write_all(content.as_bytes()));
    });

    Ok(())
}
