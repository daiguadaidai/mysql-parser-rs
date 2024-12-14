#[derive(Debug, Default, Clone)]
pub struct CIStr {
    pub origin: String,
    pub lower: String,
}

// NewCIStr creates a new CIStr.
impl CIStr {
    pub fn new(s: &str) -> CIStr {
        CIStr {
            origin: s.to_string(),
            lower: s.to_lowercase(),
        }
    }
}
