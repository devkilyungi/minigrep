#[derive(Debug, Clone)]
pub struct SearchResult {
    line_number: usize,
    line_content: String,
    matching_patterns: Vec<String>,
}

impl SearchResult {
    pub fn new(line_number: usize, line_content: String, matching_patterns: Vec<String>) -> Self {
        Self {
            line_number,
            line_content,
            matching_patterns,
        }
    }

    pub fn get_matching_patterns(&self) -> &[String] {
        &self.matching_patterns
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number
    }
    
    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
