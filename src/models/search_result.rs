//! Defines data structures for storing and displaying search results.

/// Represents a single matched line of text from a search operation.
///
/// Contains the line content, its line number in the original file,
/// and the specific patterns that matched within this line.
#[derive(Debug, Clone)]
pub struct SearchResult {
    line_number: usize,
    line_content: String,
    matching_patterns: Vec<String>,
}

impl SearchResult {
    /// Creates a new SearchResult instance.
    ///
    /// # Arguments
    ///
    /// * `line_number` - Zero-based line number where the match was found
    /// * `line_content` - The full text of the matching line
    /// * `matching_patterns` - The patterns that matched on this line
    pub fn new(line_number: usize, line_content: String, matching_patterns: Vec<String>) -> Self {
        Self {
            line_number,
            line_content,
            matching_patterns,
        }
    }

    /// Returns the patterns that matched on this line.
    pub fn get_matching_patterns(&self) -> &[String] {
        &self.matching_patterns
    }

    /// Returns the zero-based line number of this result.
    pub fn get_line_number(&self) -> usize {
        self.line_number
    }

    /// Returns the text content of this line.
    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
