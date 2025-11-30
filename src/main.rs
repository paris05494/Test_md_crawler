mod config;
mod crawler;
mod parser;
mod utils;

use spider::tokio;
use std::env;
use spider::url::Url;
use crate::crawler::fetcher::fetch_url;
use crate::parser::markdown::html_to_markdown;
use crate::utils::writer::save_file;

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    let target = if args.len() > 1 {
        args[1].clone()
    } else {
        "https://www.heygoody.com".to_string()
    };

    
    let target = if target.starts_with("http://") || target.starts_with("https://") {
        target
    } else {
        format!("https://{}", target)
    };

    
    let base = match Url::parse(&target) {
        Ok(parsed) => {
            if let Some(host) = parsed.host_str() {
                if let Some(port) = parsed.port() {
                    format!("{}://{}:{}", parsed.scheme(), host, port)
                } else {
                    format!("{}://{}", parsed.scheme(), host)
                }
            } else {
                
                target.clone()
            }
        }
        Err(_) => target.clone(),
    };

    println!("Starting Crawler for: {}\n", base);
    crawler::core::run(&base).await;

    
    if target != base {
        println!("\n[extra] Fetching provided URL: {}", target);
        if let Some(html) = fetch_url(&target).await {
            println!("[extra] Fetched provided URL");
            let md = html_to_markdown(&target, &html);
            save_file(&target, &md);
            println!("[extra] Saved provided URL to output/");
        } else {
            println!("[extra] Failed to fetch provided URL: {}", target);
        }
    }
}
