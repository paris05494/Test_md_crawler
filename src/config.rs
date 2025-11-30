use std::time::Duration;
use std::collections::HashSet;

#[derive(Clone)]
pub struct AppConfig {
    pub user_agent: String,
    pub delay_ms: u64,
    #[allow(dead_code)]
    pub timeout: Duration,
    pub spa_domains: HashSet<String>,
    pub ssr_domains: HashSet<String>,
    #[allow(dead_code)]
    pub crawl_depth: usize,
    #[allow(dead_code)]
    pub crawl_limit: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut spa_domains = HashSet::new();
        spa_domains.insert("www.heygoody.com".to_string());
        spa_domains.insert("heygoody.com".to_string());
        let ssr_domains = HashSet::new();

        Self {
            user_agent: "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)".to_string(),
            delay_ms: 1500,
            timeout: Duration::from_secs(60),
            spa_domains,
            ssr_domains,
            crawl_depth: 2,
            crawl_limit: 50,
        }
    }
}

pub fn is_spa_domain(url: &str) -> bool {
    let config = AppConfig::default();
    
    if let Ok(parsed) = spider::url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            if config.spa_domains.contains(host) {
                return true;
            }
            if config.ssr_domains.contains(host) {
                return false;
            }
        }
    }
    
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spa_detection() {
        assert!(is_spa_domain("https://www.heygoody.com/page"));
        assert!(!is_spa_domain("https://example.com/page"));
    }
}