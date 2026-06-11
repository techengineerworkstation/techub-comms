use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PHONE: Regex = Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
    static ref ROOM: Regex = Regex::new(r"^[a-zA-Z0-9_-]{1,128}$").unwrap();
    static ref DTMF: Regex = Regex::new(r"^[\d*#]{1,32}$").unwrap();
    static ref UUID: Regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    static ref TEXT_LEN: Regex = Regex::new(r"^[\s\S]{0,5000}$").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
}

pub fn phone(v: &str) -> Result<(), String> {
    if PHONE.is_match(v) { Ok(()) } else { Err(format!("Invalid phone: {}", v)) }
}
pub fn room(v: &str) -> Result<(), String> {
    if ROOM.is_match(v) { Ok(()) } else { Err(format!("Invalid room: {}", v)) }
}
pub fn dtmf(v: &str) -> Result<(), String> {
    if DTMF.is_match(v) { Ok(()) } else { Err(format!("Invalid DTMF: {}", v)) }
}
pub fn uuid(v: &str) -> Result<(), String> {
    if UUID.is_match(v) { Ok(()) } else { Err(format!("Invalid UUID: {}", v)) }
}
pub fn text(v: &str) -> Result<(), String> {
    if TEXT_LEN.is_match(v) { Ok(()) } else { Err("Text too long".into()) }
}
pub fn sanitize(v: &str) -> Result<String, String> {
    text(v)?;
    Ok(HTML_TAG.replace_all(v, "").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_phones() { assert!(phone("+14155551234").is_ok()); assert!(phone("abc").is_err()); }
    #[test]
    fn validate_rooms() { assert!(room("my-room").is_ok()); assert!(room("room with spaces").is_err()); }
    #[test]
    fn sanitize_html() { assert!(!sanitize("<script>alert(1)</script>Hi").contains("<script>")); }
}
