use anyhow::Result;
use html2md::parse_html;

pub fn convert_to_markdown(html: &str, url: &str, doc_name: &str) -> Result<String> {
    let markdown = parse_html(html);
    let attribution_comment = format!(
        "<!--\nSource: {}\nURL: {}\nLicense: MIT OR Apache-2.0\n-->\n\n",
        doc_name, url
    );
    let attribution_visible = format!(
        "\n\n---\n**Source**: {}\n**URL**: {}\n**License**: MIT OR Apache-2.0\n",
        doc_name, url
    );
    Ok(format!(
        "{}{}{}",
        attribution_comment, markdown, attribution_visible
    ))
}
