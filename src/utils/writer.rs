use spider::url::Url;
use std::fs;
use std::path::Path;

pub fn save_file(url: &str, content: &str) {
    let parsed = Url::parse(url).unwrap();
    let domain = parsed.host_str().unwrap_or("unknown");
    let slug = parsed.path().replace("/", "_");
    let filename = format!("output/{}{}.md", domain, slug);

    if let Some(parent) = Path::new(&filename).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _ = fs::write(filename, content);
}
