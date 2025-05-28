# 📚 rust-doc-scraper

A fast, idiomatic Rust tool for scraping and converting official Rust documentation sites into Markdown — with automatic attribution headers and offline-friendly output.

Built for maintainers of AI agents, documentation tools, or GPTs that use content from `rust-lang.org`, `docs.rs`, or other community-authored Rust books and sites.

---

## 🚀 Features

- 🔍 **Scrapes HTML pages** from Rust ecosystem documentation sites
- 📄 **Converts to Markdown** using customizable rules
- 🖋️ **Injects attribution headers** automatically
- 📂 **Outputs Markdown to structured folders**
- 🦀 100% **Rust-native**, fast and parallelizable

---

## 📘 Supported Docs & Books

The following sources are currently scraped:

| Doc Name                       | Source URL                                                                                             |
| ------------------------------ | ------------------------------------------------------------------------------------------------------ |
| The Rust Book                  | [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)                                     |
| Rust by Example                | [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)               |
| The Cargo Book                 | [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)                                   |
| The Rustonomicon               | [https://doc.rust-lang.org/nomicon/](https://doc.rust-lang.org/nomicon/)                               |
| The Async Book                 | [https://rust-lang.github.io/async-book/](https://rust-lang.github.io/async-book/)                     |
| The Clippy Book                | [https://rust-lang.github.io/rust-clippy/current/](https://rust-lang.github.io/rust-clippy/current/)   |
| Error Index                    | [https://doc.rust-lang.org/error\_codes/](https://doc.rust-lang.org/error_codes/)                      |
| Rust API Guidelines            | [https://rust-lang.github.io/api-guidelines/](https://rust-lang.github.io/api-guidelines/)             |
| The Rust and WebAssembly Book  | [https://rustwasm.github.io/book/](https://rustwasm.github.io/book/)                                   |
| Tokio Documentation            | [https://docs.rs/tokio/latest/tokio/](https://docs.rs/tokio/latest/tokio/)                             |
| Axum Documentation             | [https://docs.rs/axum/latest/axum/](https://docs.rs/axum/latest/axum/)                                 |
| Leptos Book                    | [https://book.leptos.dev/](https://book.leptos.dev/)                                                   |
| Embedded Rust Book             | [https://docs.rust-embedded.org/book/](https://docs.rust-embedded.org/book/)                           |
| The Little Book of Rust Macros | [https://danielkeep.github.io/tlborm/book/](https://danielkeep.github.io/tlborm/book/)                 |
| Too Many Linked Lists          | [https://rust-unofficial.github.io/too-many-lists/](https://rust-unofficial.github.io/too-many-lists/) |

---

## ⚙️ Customizing the Docs to Scrape

You can customize which documentation sites the scraper pulls from by editing the source list in:

```
src/targets.rs
```

Inside, you'll find a function like:

```rust
pub fn get_scrape_targets() -> HashMap<String, String> {
    HashMap::from([
        ("The Rust Programming Language Book".into(), "https://doc.rust-lang.org/book/".into()),
        ("Tokio Documentation".into(), "https://docs.rs/tokio/latest/tokio/".into()),
        // ...
    ])
}
```

You can:

* ✅ **Add new entries** to scrape new Rust documentation sites
* ❌ **Remove entries** if you don’t need certain sources
* ✏️ **Rename entries** (keys are just used for folder names)

Changes take effect next time you run the scraper.

---

## 🛠️ Usage

### 1. 📦 Build the tool

```bash
cargo build --release
```

### 2. 🧪 Run the scraper

To scrape all configured sources and output Markdown into `output/`:

```bash
cargo run --release
```

> If you only want to run specific modules, you can comment out others in `main.rs`.

### 3. 📁 Output

* Markdown files will be saved in folder:
  `./scraped_docs/`

* Attribution headers are prepended like:

```markdown
<!--
Source: The Rust Book - https://doc.rust-lang.org/book/
License: MIT OR Apache-2.0
-->
```

---

## 📜 Attribution & License Info

* All scraped content includes source URL and license attribution in each `.md` or `.rs` file.
* All sources currently use dual `MIT` or `Apache-2.0` licenses.
* You can find complete references in [`ATTRIBUTION.md`](./ATTRIBUTION.md).

---

## ⚙️ Project Structure

```
src/
├── main.rs             # Entry point
├── scrape.rs           # Web scraping and HTML-to-Markdown logic
├── attribute_md.rs     # Attribution for .md files
├── attribute_rs.rs     # Attribution for .rs files
├── utils.rs            # Helper functions
output/                 # Final Markdown output
```

---

## 🧰 Requirements

* Rust 1.72+ (tested)
* OpenSSL (for crates using `reqwest` on some systems)

### Linux (Ubuntu/Debian)

```bash
sudo apt install pkg-config libssl-dev
```

---

## 🧪 Testing & Linting

```bash
cargo fmt     # Format
cargo clippy  # Lint
cargo test    # (Future: Add test suite)
```

---

## 🪪 License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](./LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))

You may choose either license.

Scraped documentation content retains the license of its original source (typically MIT OR Apache-2.0).  
See [`ATTRIBUTION.md`](./ATTRIBUTION.md) for source-specific license references.


---

## 🙋 Contributing

PRs welcome — especially for:

* New doc sources
* Better markdown cleaning
* Language-specific scraping (i18n)

---

**Last updated:** 2025-05-25
