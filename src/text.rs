pub fn has_wrap(s: &str) -> bool {
    s.contains("\n") || s.contains("\r") || s.contains("\u{2028}") || s.contains("\u{2029}")
}
