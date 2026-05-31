// src/utils/uri.rs
pub fn normalize_uri_prefix(input: Option<&str>) -> String {
    let raw = input.unwrap_or("").trim();
    if raw.is_empty() || raw == "/" {
        return String::new();
    }
    let trimmed = raw.trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("/{}", trimmed)
    }
}
