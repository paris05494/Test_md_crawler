use super::fetcher::fetch_url;
use super::robots::{
    fetch_sitemap_recursive, get_sitemaps_from_robots, get_urls_from_sitemap_direct, crawl_native,
};
use crate::parser::markdown::html_to_markdown;
use crate::utils::writer::save_file;
use std::collections::HashSet;

pub async fn run(domain: &str) {
    println!("\n========================================");
    println!("   Starting Crawler for: {}", domain);
    println!("========================================\n");

    let mut target_urls = Vec::new();

    println!("\n[Step 1] Fetching robots.txt...");
    let sitemaps = get_sitemaps_from_robots(domain).await;

    if !sitemaps.is_empty() {
        println!("Found {} sitemap(s) in robots.txt", sitemaps.len());

        println!("\n[Step 2] Parsing sitemaps recursively...");
        let mut visited_sitemaps = HashSet::new();
        for sitemap_url in sitemaps {
            let urls = fetch_sitemap_recursive(&sitemap_url, &mut visited_sitemaps, 0).await;
            println!("   Found {} URLs from: {}", urls.len(), sitemap_url);
            target_urls.extend(urls);
        }
    } else {
        println!("No sitemaps found in robots.txt");

        println!("\n[Step 3] Trying direct /sitemap.xml fallback...");
        let fallback_urls = get_urls_from_sitemap_direct(domain).await;
        if !fallback_urls.is_empty() {
            println!("Found {} URLs from direct sitemap.xml", fallback_urls.len());
            target_urls.extend(fallback_urls);
        } else {
            println!("No sitemap.xml found, using native spider crawl...");
            println!("\n[Step 4] Native crawl from domain root...");
            let crawled_urls = crawl_native(domain, 2, 50).await;
            println!("Native crawl found {} URLs", crawled_urls.len());
            target_urls.extend(crawled_urls);
        }
    }

    let mut unique_urls: Vec<String> = target_urls.into_iter().collect::<HashSet<_>>().into_iter().collect();
    unique_urls.sort();

    if unique_urls.is_empty() {
        println!("No URLs found! Fallback to domain root.");
        unique_urls.push(domain.to_string());
    } else {
        println!("\nTotal unique URLs to crawl: {}", unique_urls.len());
    }

    println!("\n[Step 5-7] Fetching and converting to Markdown...");
    let limit = 5;
    
    for (i, url) in unique_urls.iter().take(limit).enumerate() {
        println!("\n[{}/{}] Processing: {}", i + 1, limit.min(unique_urls.len()), url);

        if let Some(html) = fetch_url(url).await {
            println!("   Fetched HTML");
            let markdown = html_to_markdown(url, &html);
            save_file(url, &markdown);
            println!("   Saved to output/");
        } else {
            println!("   Failed to fetch");
        }
    }

    println!("\nCrawler finished!\n");
}
