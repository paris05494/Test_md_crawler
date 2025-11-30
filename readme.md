# md_crawler

Overview
 - Project: md_crawler
 - Language: Rust (edition 2021)
 - Purpose: A small example web crawler that discovers URLs via robots.txt → sitemaps → native crawl fallback, fetches HTML, converts pages to Markdown, and saves them to disk.

Key features
 - Finds sitemaps listed in `robots.txt` and recursively parses sitemap XML files
 - Falls back to a native spider crawl when sitemaps are unavailable
 - Fetches pages primarily with the `spider` crate and falls back to a direct HTTP GET via `reqwest` when `spider` does not return page content
 - Converts HTML to basic Markdown and saves files under the `output/` directory

Lightweight example web crawler written in Rust. md_crawler discovers pages via robots.txt and sitemaps (with a native spider fallback), fetches HTML, converts pages to Markdown, and writes the results to the `output/` folder.

Installation

Clone and build:

```bash
git clone <repo-url>
cd md_crawler
cargo build --release
```

Quick start

Run the crawler against a single URL (example):

```bash
cargo run -- https://httpbin.org/html
```

Basic usage

- Pass a target URL as the first argument. The program normalizes to the site root and uses that root for robots/sitemap discovery.
- Output Markdown files are written to `output/` (one file per page).

Configuration

Edit `src/config.rs` to change defaults:

- `user_agent` — default user agent string used by the spider
- `delay_ms` — spider delay (milliseconds) between requests
- `timeout` — timeout used by the fallback HTTP client
- `spa_domains` — domains treated as SPAs (used to prefer a Chrome-like UA)

Project layout

- `Cargo.toml` — dependencies (`spider`, `tokio`, `reqwest`)
- `src/main.rs` — CLI entry and URL normalization
- `src/config.rs` — configuration defaults
- `src/crawler/` — crawler logic
	- `core.rs` — orchestration pipeline (robots → sitemap → fallback → fetch → parse → save)
	- `robots.rs` — robots.txt and sitemap parsing, native crawl
	- `fetcher.rs` — page fetching (primary: spider; fallback: reqwest)
# md_crawler

Usage

Clone and enter the repository:

```cmd
git clone <repo-url>
cd md_crawler
```

Build / check:

```cmd
cargo build --release
cargo check
```

Run the crawler (provide a target URL):

```cmd
cargo run -- https://httpbin.org/html
cargo run -- https://example.com
```

Output

- Generated Markdown files are written to the `output/` directory (one file per page).
