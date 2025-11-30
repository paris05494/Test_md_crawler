use spider::website::Website;
use crate::config::AppConfig;
use std::collections::HashSet;

pub async fn get_sitemaps_from_robots(base_url: &str) -> Vec<String> {
    let robots_url = format!("{}/robots.txt", base_url.trim_end_matches('/'));
    println!("[robots] Fetching: {}", robots_url);

    let mut website = Website::new(&robots_url);
    website.configuration.user_agent = Some(Box::new(AppConfig::default().user_agent.into()));
    website.with_depth(0);
    
    website.scrape().await;

    let mut sitemaps = Vec::new();

    if let Some(pages) = website.get_pages() {
        if let Some(page) = pages.first() {
            let content = page.get_html();
            for line in content.lines() {
                let trimmed = line.trim();
                let lower = trimmed.to_lowercase();
                if lower.starts_with("sitemap:") {
                    let url = trimmed[8..].trim();
                    if !url.is_empty() {
                        sitemaps.push(url.to_string());
                        println!("[robots] Found sitemap: {}", url);
                    }
                }
            }
        }
    }

    sitemaps
}

pub async fn fetch_sitemap_recursive(
    sitemap_url: &str,
    visited: &mut HashSet<String>,
    depth: usize
) -> Vec<String> {
    if visited.contains(sitemap_url) || depth > 3 {
        return Vec::new();
    }
    visited.insert(sitemap_url.to_string());

    println!("[sitemap] Parsing (depth {}): {}", depth, sitemap_url);

    let mut website = Website::new(sitemap_url);
    website.configuration.user_agent = Some(Box::new(AppConfig::default().user_agent.into()));
    website.with_depth(0);
    website.scrape().await;

    let mut found_urls = Vec::new();

    if let Some(pages) = website.get_pages() {
        if let Some(page) = pages.first() {
            let content = page.get_html();

            let parts: Vec<&str> = content.split("<loc>").collect();
            for part in parts.iter().skip(1) {
                if let Some(end_pos) = part.find("</loc>") {
                    let url = part[..end_pos].trim().to_string();

                    if !url.is_empty() {
                        if url.ends_with(".xml") || url.contains("sitemap") {
                            println!("[sitemap] Found nested sitemap: {}", url);
                            let children = Box::pin(fetch_sitemap_recursive(&url, visited, depth + 1)).await;
                            found_urls.extend(children);
                        } else {
                            found_urls.push(url);
                        }
                    }
                }
            }
        }
    }

    found_urls
}

pub async fn get_urls_from_sitemap_direct(base_url: &str) -> Vec<String> {
    let sitemap_url = format!("{}/sitemap.xml", base_url.trim_end_matches('/'));
    println!("[sitemap-fallback] Trying direct sitemap: {}", sitemap_url);

    let mut visited = HashSet::new();
    fetch_sitemap_recursive(&sitemap_url, &mut visited, 0).await
}

pub async fn crawl_native(base_url: &str, max_depth: usize, limit: usize) -> Vec<String> {
    println!("[native-crawl] Starting native spider crawl from: {}", base_url);
    
    let mut website = Website::new(base_url);
    website.configuration.user_agent = Some(Box::new(AppConfig::default().user_agent.into()));
    website.with_depth(max_depth);
    website.configuration.delay = AppConfig::default().delay_ms;

    website.scrape().await;

    let mut urls = Vec::new();
    
    if let Some(pages) = website.get_pages() {
        for page in pages.iter().take(limit) {
            let url = page.get_url();
            println!("[native-crawl] Found URL: {}", url);
            urls.push(url.to_string());
        }
    }

    if urls.is_empty() {
        println!("[native-crawl] No URLs found via spider, fallback to root domain");
        urls.push(base_url.to_string());
    }

    urls
}