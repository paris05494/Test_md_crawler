use spider::website::Website;
use crate::config::{AppConfig, is_spa_domain};
use reqwest::Client;
use std::time::Duration as StdDuration;

async fn fetch_with_ua(url: &str, ua: &str, delay_ms: u64) -> Option<String> {
    let mut website = Website::new(url);
    website.configuration.user_agent = Some(Box::new(ua.into()));
    website.with_depth(0);
    website.configuration.delay = delay_ms;

    let cfg = AppConfig::default();
    website.configuration.crawl_timeout = Some(cfg.timeout);
    website.configuration.respect_robots_txt = false;

    println!("[fetcher] scraping: {} (UA: {}, delay: {}ms)", url, ua, delay_ms);

    website.scrape().await;

    if let Some(pages) = website.get_pages() {
        println!("[fetcher] spider returned {} page(s)", pages.len());
        for (idx, page) in pages.iter().take(5).enumerate() {
            println!("[fetcher]  page[{}]: {}", idx, page.get_url());
            let html = page.get_html();
            println!("[fetcher]   html.len = {}", html.len());
            if !html.is_empty() {
                println!("[fetcher] fetched {} bytes", html.len());
                return Some(html.to_string());
            }
        }
    } else {
        println!("[fetcher] spider.get_pages() returned None");
    }

    println!("[fetcher] falling back to direct HTTP GET for: {}", url);
    let client = match Client::builder()
        .user_agent(ua)
        .timeout(StdDuration::from_secs(AppConfig::default().timeout.as_secs()))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            println!("[fetcher] reqwest client build error: {}", e);
            return None;
        }
    };

    match client.get(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("[fetcher] reqwest fetched {} bytes", text.len());
                if !text.is_empty() {
                    return Some(text);
                }
            }
            Err(e) => println!("[fetcher] reqwest read error: {}", e),
        },
        Err(e) => println!("[fetcher] reqwest request error: {}", e),
    }

    None
}

pub async fn fetch_url(url: &str) -> Option<String> {
    let config = AppConfig::default();

    let start_ua = if is_spa_domain(url) {
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
    } else {
        &config.user_agent
    };

    let fallback_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36";

    let mut attempt = 0usize;
    let max_attempts = 3usize;
    let mut backoff = 500u64;

    println!("[fetcher] Fetching: {}", url);

    while attempt < max_attempts {
        attempt += 1;

        
        let ua = if attempt == 1 { start_ua } else { fallback_ua };

        if let Some(html) = fetch_with_ua(url, ua, config.delay_ms).await {
            return Some(html);
        }

        println!("[fetcher] attempt {} failed for {} — backing off {}ms", attempt, url, backoff);
        spider::tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
        backoff = std::cmp::min(backoff * 2, 5000);
    }

    println!("[fetcher] Failed to fetch content from: {} after {} attempts", url, max_attempts);
    None
}

#[allow(dead_code)]
pub async fn fetch_urls_batch(urls: &[String]) -> Vec<(String, String)> {
    let mut results = Vec::new();

    for url in urls {
        if let Some(html) = fetch_url(url).await {
            results.push((url.clone(), html));
        }
    }

    results
}