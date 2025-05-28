# 📄 ATTRIBUTION

This project (`rust-doc-scraper`) fetches, converts, and stores selected Rust documentation sources in Markdown format to support offline usage, AI integration, and code analysis tools.

The original content belongs to its respective maintainers. We **do not claim ownership**. All scraped sources are licensed under **MIT OR Apache-2.0**, and content reuse follows the licensing terms of each project.

---

## 🧾 Attribution Strategy

To ensure compliance with open-source licenses and provide clear traceability of each document's origin, every scraped Markdown file includes:

- ✅ A **prepended HTML comment** at the top of the file, suitable for parsing and invisible to readers:

  ```md
  <!--
  Source: The Rust Book - https://doc.rust-lang.org/book/
  License: MIT OR Apache-2.0
  -->
  ```

* ✅ An **appended visible Markdown block** at the bottom of the file:

  ```md
  ---
  **Source**: the_rust_book  
  **URL**: https://doc.rust-lang.org/book/  
  **License**: MIT OR Apache-2.0
  ```

This dual approach ensures that:

* **AI systems** and other tools processing content are informed of origin and license.
* **Humans** consuming the Markdown manually also receive full attribution.

---

## 📘 Attributed Sources

The following official Rust documentation sources are currently scraped:

| Doc Name                       | Source URL                                                                                             | License           |
| ------------------------------ | ------------------------------------------------------------------------------------------------------ | ----------------- |
| The Rust Book                  | [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)                                     | MIT OR Apache-2.0 |
| Rust by Example                | [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)               | MIT OR Apache-2.0 |
| The Cargo Book                 | [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)                                   | MIT OR Apache-2.0 |
| The Rustonomicon               | [https://doc.rust-lang.org/nomicon/](https://doc.rust-lang.org/nomicon/)                               | MIT OR Apache-2.0 |
| The Async Book                 | [https://rust-lang.github.io/async-book/](https://rust-lang.github.io/async-book/)                     | MIT OR Apache-2.0 |
| The Clippy Book                | [https://rust-lang.github.io/rust-clippy/current/](https://rust-lang.github.io/rust-clippy/current/)   | MIT OR Apache-2.0 |
| Error Index                    | [https://doc.rust-lang.org/error\_codes/](https://doc.rust-lang.org/error_codes/)                      | MIT OR Apache-2.0 |
| Rust API Guidelines            | [https://rust-lang.github.io/api-guidelines/](https://rust-lang.github.io/api-guidelines/)             | MIT OR Apache-2.0 |
| The Rust and WebAssembly Book  | [https://rustwasm.github.io/book/](https://rustwasm.github.io/book/)                                   | MIT OR Apache-2.0 |
| Tokio Documentation            | [https://docs.rs/tokio/latest/tokio/](https://docs.rs/tokio/latest/tokio/)                             | MIT OR Apache-2.0 |
| Axum Documentation             | [https://docs.rs/axum/latest/axum/](https://docs.rs/axum/latest/axum/)                                 | MIT OR Apache-2.0 |
| Leptos Book                    | [https://book.leptos.dev/](https://book.leptos.dev/)                                                   | MIT OR Apache-2.0 |
| Embedded Rust Book             | [https://docs.rust-embedded.org/book/](https://docs.rust-embedded.org/book/)                           | MIT OR Apache-2.0 |
| The Little Book of Rust Macros | [https://danielkeep.github.io/tlborm/book/](https://danielkeep.github.io/tlborm/book/)                 | MIT OR Apache-2.0 |
| Too Many Linked Lists          | [https://rust-unofficial.github.io/too-many-lists/](https://rust-unofficial.github.io/too-many-lists/) | MIT OR Apache-2.0 |

---

## 🔐 Licensing Summary

* All scraped documentation is **owned by its original authors**.
* This repository’s **code** is licensed under **MIT OR Apache-2.0**.
* Scraped **content** is provided **with full attribution and license compliance**, in support of educational and research goals.

---

If you are a maintainer of one of the above sources and have concerns or requests, please open an issue or contact the maintainers directly.
