use std::collections::HashMap;

pub fn get_scrape_targets() -> HashMap<String, String> {
    HashMap::from([
        (
            "Rust MCP SDK".into(),
            "https://docs.rs/rust-mcp-sdk/latest/rust_mcp_sdk/".into(),
        ),
        (
            "The Rust Programming Language Book".into(),
            "https://doc.rust-lang.org/book/".into(),
        ),
        (
            "The Cargo Book".into(),
            "https://doc.rust-lang.org/cargo/".into(),
        ),
        (
            "Rust by Example".into(),
            "https://doc.rust-lang.org/rust-by-example/".into(),
        ),
        (
            "The Rustonomicon".into(),
            "https://doc.rust-lang.org/nomicon/".into(),
        ),
        (
            "Async Programming in Rust".into(),
            "https://rust-lang.github.io/async-book/".into(),
        ),
        (
            "Clippy Lints".into(),
            "https://rust-lang.github.io/rust-clippy/current/".into(),
        ),
        (
            "Rust Error Index".into(),
            "https://doc.rust-lang.org/error_codes/".into(),
        ),
        (
            "Rust API Guidelines".into(),
            "https://rust-lang.github.io/api-guidelines/".into(),
        ),
        (
            "Axum Docs and Examples".into(),
            "https://docs.rs/axum/latest/axum/".into(),
        ),
        ("Leptos Book".into(), "https://book.leptos.dev/".into()),
        (
            "The Embedded Rust Book".into(),
            "https://docs.rust-embedded.org/book/".into(),
        ),
        (
            "The Rust and WebAssembly Book".into(),
            "https://rustwasm.github.io/book/".into(),
        ),
        (
            "The Little Book of Rust Macros".into(),
            "https://danielkeep.github.io/tlborm/book/".into(),
        ),
        (
            "Too Many Linked Lists".into(),
            "https://rust-unofficial.github.io/too-many-lists/".into(),
        ),
    ])
}
