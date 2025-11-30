# md_crawler


## Clone and build

```bash
git clone <repo-url>
cd md_crawler
cargo build --release
```

## Installation

### Prerequisites

- Rust toolchain (recommended: Rust 1.70+ / 2024 edition)
- Chrome or Chromium (needed for SPA crawling when using the Chrome fetcher)

### Build

```bash
cargo build --release
```

## Project layout (source overview)

```
src/
├── main.rs              # entry point + CLI
├── config.rs            # application configuration and defaults
├── crawler/
│   ├── mod.rs
│   ├── core.rs          # orchestration pipeline (robots -> sitemap -> crawl -> fetch -> parse -> save)
│   ├── robots.rs        # robots.txt and sitemap helpers
│   └── fetcher.rs       # page fetching and spider integration
├── parser/
│   ├── mod.rs
│   └── markdown.rs      # HTML -> Markdown conversion
└── utils/
    ├── mod.rs
    └── writer.rs        # write Markdown files to `output/`
```

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

For the detailed runtime defaults, see `src/config.rs`.
Run the crawler against a single URL (example):

```bash
cargo run -- https://www.heygoody.com
```
