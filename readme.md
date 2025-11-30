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
That's it — the repository has only the usage instructions here. For development or debugging, run `cargo check` and then `cargo run -- <url>` to produce `output/` files.
Quick start
## Installation

### Prerequisites

- Rust toolchain (recommended: Rust 1.70+ / 2024 edition)
- Chrome or Chromium (needed for SPA crawling when using the Chrome fetcher)

### Build

```bash
cargo build --release
```

## Usage

### Basic

```bash
# Crawl a site (auto-detects SPA vs SSR where possible)
cargo run -- "https://www.example.com"

# Force Chrome mode for SPA-like sites
cargo run -- "https://react-app.example"

# Force HttpRequest mode for SSR sites
cargo run -- "https://ssr-site.example"
```

### Test cases (examples)

```bash
# Site with robots.txt
cargo run -- "https://www.muangthai.co.th/th"

# Direct sitemap.xml test
cargo run -- "https://thewhitemarketing.com/"

# Native crawler mode (no sitemap)
cargo run -- "https://www.rust-lang.org/"

# SSR example
cargo run -- "https://thewhitemarketing.com/"

# SPA example
cargo run -- "https://leerob.com/"
```

### App configuration (`src/config/app.yaml`)

Create or edit `src/config/app.yaml` to tune crawler behavior:

```yaml
depth: 1                          # crawl depth
user_agent: "SSS/1.0"            # UA string used for requests
delay_ms: 50                      # delay between requests (ms)
max_pages: 5                      # maximum pages to crawl
fetch_mode: "Chrome"             # forced fetch mode: "Chrome" or "HttpRequest"
whitelist_path: "src/config/whitelist.yaml"  # domain whitelist file
sitemap_max_depth: 5              # max sitemap recursion depth
max_sitemap_urls: 5               # max URLs extracted from sitemaps
```

### Domain whitelist (`src/config/whitelist.yaml`)

Use a domain whitelist to control how domains are fetched. Example:

```yaml
auto_mode: true
default_mode: "SPA"
match_on: "domain_only"

whitelist:
	- domain: "www.heygoody.com"
		mode: "SPA"
		handler: "chrome"
		match: "exact"

	- domain: "reactjs.org"
		mode: "SPA"
		handler: "chrome"
		match: "subdomain"
```

## Project layout (source overview)

```
src/
├── main.rs              # entry point
├── config/
│   ├── mod.rs
│   ├── config.rs        # config structs and defaults
│   ├── app.yaml         # app-level defaults (optional)
│   └── whitelist.yaml   # domain whitelist
└── crawler/
		├── mod.rs
		├── core.rs          # main crawling orchestration
		├── robots.rs        # robots.txt and sitemap helpers
		├── fetcher.rs       # fetch-mode dispatcher (spider / http / chrome)
		├── chrome_fetcher.rs# Chrome-based fetching (optional)
		├── parser.rs        # HTML -> Markdown conversion
		├── writer.rs        # write Markdown to `output/`
		└── domain_detector.rs # domain classification (SPA/SSR)
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

## Configuration options (defaults)

| Option | Description | Default |
|--------|-------------|---------|
| `depth` | crawling depth | `1` |
| `user_agent` | HTTP user agent string | `"SSS/1.0"` |
| `delay_ms` | delay between requests (ms) | `50` |
| `max_pages` | maximum pages to crawl | `5` |
| `fetch_mode` | force fetch mode (`Chrome` or `HttpRequest`) | `"Chrome"` |
| `sitemap_max_depth` | max sitemap nesting depth | `5` |
| `max_sitemap_urls` | max URLs to extract from sitemaps | `5` |

For the detailed runtime defaults, see `src/config.rs`.
Run the crawler against a single URL (example):

```bash
cargo run -- https://www.heygoody.com
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
cargo run -- https://www.heygoody.com
cargo run -- https://example.com
```

Output

- Generated Markdown files are written to the `output/` directory (one file per page).
