use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub async fn extract_subpages(base_url: &str) -> anyhow::Result<HashMap<String, String>> {
    let body = reqwest::get(base_url).await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    let base = Url::parse(base_url)?;
    let mut pages = HashMap::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.starts_with("http") || href.starts_with('#') {
                continue;
            }
            if href.ends_with(".html") {
                let full_url = base.join(href)?;
                let slug = href.replace("/", "_").replace(".html", "");
                pages.insert(slug, full_url.to_string());
            }
        }
    }

    Ok(pages)
}
