pub fn html_to_markdown(url: &str, html: &str) -> String {
    let mut markdown = format!("# Source: {}\n\n", url);
    let mut in_ul = false;
    let mut in_ol = false;
    let mut ol_counter = 1;
    
    markdown.push_str(&parse_html_to_markdown(html, &mut in_ul, &mut in_ol, &mut ol_counter));
    markdown
}

fn parse_html_to_markdown(html: &str, in_ul: &mut bool, in_ol: &mut bool, ol_counter: &mut usize) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = html.chars().collect();

    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(end) = chars[i..].iter().position(|&c| c == '>') {
                let end_pos = i + end;
                let tag_content = chars[i + 1..end_pos].iter().collect::<String>();
                
                if tag_content.starts_with('/') {
                    
                    let tag_name = extract_tag_name(&tag_content[1..]);
                    match tag_name.as_str() {
                        "ul" => *in_ul = false,
                        "ol" => *in_ol = false,
                        "p" => result.push('\n'),
                        _ => {}
                    }
                } else {
                    
                    let tag_name = extract_tag_name(&tag_content);
                    
                    match tag_name.as_str() {
                        "h1" => {
                            result.push_str("\n# ");
                        }
                        "h2" => {
                            result.push_str("\n## ");
                        }
                        "h3" => {
                            result.push_str("\n### ");
                        }
                        "h4" => {
                            result.push_str("\n#### ");
                        }
                        "p" => {
                            if !result.ends_with("\n\n") {
                                result.push('\n');
                            }
                        }
                        "br" => {
                            result.push('\n');
                        }
                        "ul" => {
                            *in_ul = true;
                            result.push('\n');
                        }
                        "ol" => {
                            *in_ol = true;
                            *ol_counter = 1;
                            result.push('\n');
                        }
                        "li" => {
                            if *in_ul {
                                result.push_str("- ");
                            } else if *in_ol {
                                result.push_str(&format!("{}. ", ol_counter));
                                *ol_counter += 1;
                            }
                        }
                        "strong" | "b" => {
                            result.push_str("**");
                        }
                        "em" | "i" => {
                            result.push_str("*");
                        }
                        "blockquote" => {
                            result.push_str("\n> ");
                        }
                        "a" => {
                            if let Some(href) = extract_attribute(&tag_content, "href") {
                                result.push('[');
                                result.push_str(&format!("]({})", href));
                            }
                        }
                        "img" => {
                            if let Some(src) = extract_attribute(&tag_content, "src") {
                                if let Some(alt) = extract_attribute(&tag_content, "alt") {
                                    result.push_str(&format!("![{}]({})\n", alt, src));
                                } else {
                                    result.push_str(&format!("![image]({})\n", src));
                                }
                            }
                        }
                        "code" => {
                            result.push('`');
                        }
                        "pre" => {
                            result.push_str("\n```\n");
                        }
                        _ => {}
                    }
                }

                i = end_pos + 1;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else if chars[i] == '&' {
            
            if let Some(entity) = parse_entity(&chars[i..]) {
                result.push_str(&entity);
                i += entity.len() + 1;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result.trim().to_string()
}

fn extract_tag_name(tag_content: &str) -> String {
    tag_content
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_lowercase()
}

fn extract_attribute(tag_content: &str, attr_name: &str) -> Option<String> {
    let lower = tag_content.to_lowercase();
    let attr_pattern = format!("{}=", attr_name);
    
    if let Some(start) = lower.find(&attr_pattern) {
        let after_eq = &tag_content[start + attr_pattern.len()..];
        
        if after_eq.starts_with('"') || after_eq.starts_with('\'') {
            let quote = &after_eq[0..1];
            if let Some(end) = after_eq[1..].find(quote) {
                return Some(after_eq[1..end + 1].to_string());
            }
        } else {
            
            if let Some(end) = after_eq.find(|c: char| c.is_whitespace() || c == '>') {
                return Some(after_eq[..end].to_string());
            } else {
                return Some(after_eq.to_string());
            }
        }
    }
    
    None
}

fn parse_entity(chars: &[char]) -> Option<String> {
    if chars.len() < 2 || chars[0] != '&' {
        return None;
    }

    let mut entity = String::from("&");
    let mut i = 1;

    while i < chars.len() && i < 10 {
        entity.push(chars[i]);
        if chars[i] == ';' {
            break;
        }
        i += 1;
    }

    match entity.as_str() {
        "&amp;" => Some(" ".to_string()),
        "&lt;" => Some("<".to_string()),
        "&gt;" => Some(">".to_string()),
        "&quot;" => Some("\"".to_string()),
        "&apos;" => Some("'".to_string()),
        "&nbsp;" => Some(" ".to_string()),
        _ => None,
    }
}
