pub struct SearchResult {
    line_number: usize,
    line_content: String,
}

impl SearchResult {
    pub fn new(line_number: usize, line_content: String) -> Self {
        Self {
            line_number,
            line_content,
        }
    }
}

impl SearchResult {
    pub fn display(&self) {
        println!("Line {}: {}", self.line_number + 1, self.line_content);
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number + 1
    }

    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
