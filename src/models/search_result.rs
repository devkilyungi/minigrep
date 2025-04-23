#[derive(Debug, Clone)]
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
    pub fn display(&self, query: &str, ignore_case: bool) {
        let mut highlighted_line = String::new();
        let mut last_index = 0;

        if ignore_case {
            let query_lowercase = query.to_lowercase();
            let content_lowercase = self.line_content.to_lowercase();
            let mut indices = Vec::new();

            // Find all of the occurrences in the lowercase version
            let mut start = 0;
            while let Some(position) = content_lowercase[start..].find(&query_lowercase) {
                let absolute_position = start + position;
                indices.push((
                    absolute_position,
                    &self.line_content[absolute_position..(absolute_position + query.len())],
                ));
                start = absolute_position + 1;
            }
            
            // Build the highlighted string
            for (start, part) in indices {
                highlighted_line.push_str(&self.line_content[last_index..start]);
                highlighted_line.push_str("\x1b[1;31m"); // Bold red text
                highlighted_line.push_str(part);
                highlighted_line.push_str("\x1b[0m"); // Reset formatting
                last_index = start + part.len();
            }
        } else {
            for (start, part) in self.line_content.match_indices(query) {
                highlighted_line.push_str(&self.line_content[last_index..start]);
                highlighted_line.push_str("\x1b[1;31m"); // Bold red text
                highlighted_line.push_str(part);
                highlighted_line.push_str("\x1b[0m"); // Reset formatting
                last_index = start + part.len();
            }
        }

        // Add any remaining text after the last match
        highlighted_line.push_str(&self.line_content[last_index..]);

        println!("Line {}: {}", self.line_number + 1, highlighted_line);
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number + 1
    }

    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
