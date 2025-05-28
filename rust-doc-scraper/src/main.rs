mod convert;
mod extract;
mod targets;
mod utils;
mod writer;

use crate::convert::convert_to_markdown;
use crate::extract::extract_subpages;
use crate::targets::get_scrape_targets;
use crate::utils::to_snake_case;
use crate::writer::write_markdown_batch;

use anyhow::Result;
use futures::stream::{FuturesUnordered, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    let targets = get_scrape_targets();
    let output_root = "./scraped_docs";

    for (name, url) in targets {
        println!("📘 Scraping: {} ({})", name, url);
        let subpages = extract_subpages(&url).await?;

        let mut tasks = FuturesUnordered::new();
        for (slug, page_url) in subpages {
            let doc_name = name.clone();
            tasks.push(tokio::spawn(async move {
                match reqwest::get(&page_url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => convert_to_markdown(&body, &page_url, &doc_name)
                            .map(|md| Some((slug, md)))
                            .unwrap_or(None),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            }));
        }

        let mut results = Vec::new();
        while let Some(res) = tasks.next().await {
            if let Ok(Some((slug, content))) = res {
                results.push((slug, content));
            }
        }

        let dir = format!("{}/{}", output_root, to_snake_case(&name));
        write_markdown_batch(results, &dir)?;
    }

    println!("✅ Done scraping.");
    Ok(())
}
