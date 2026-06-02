// src/utils/sanitize.rs
pub fn sanitize_filename(input: &str) -> Option<String> {
    let s = input.trim();
    if s.is_empty() || s.contains('/') || s.contains('\\') || s.contains("..") {
        return None;
    }
    if !s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ".-_()+%".contains(c))
    {
        return None;
    }
    Some(s.to_string())
}
