//! Provides functionality for tracking and displaying search statistics.

use crate::models::{Config, SearchResult};
use std::time::Duration;

/// Tracks statistics about a search operation.
///
/// Collects metrics such as number of files searched, lines processed,
/// matches found, and time taken for the search operation.
pub struct SearchStats {
    pub query: String,
    pub total_lines: usize,
    pub total_matches: usize,
    pub files_searched: usize,
    pub duration: Duration,
}

impl SearchStats {
    /// Initializes a new SearchStats instance from a Config.
    ///
    /// # Arguments
    ///
    /// * `config` - The search configuration to extract initial data from
    pub fn init_stats(config: &Config) -> Self {
        SearchStats {
            query: config.query.clone(),
            total_lines: 0,
            total_matches: 0,
            files_searched: 0,
            duration: Duration::default(),
        }
    }

    /// Updates the match count based on search results.
    ///
    /// # Arguments
    ///
    /// * `results` - The search results to count matches from
    /// * `config` - The search configuration used
    pub fn update_match_count(&mut self, results: &[SearchResult], config: &Config) {
        // Check if we're using regex patterns
        let regex_indicators = [
            '*', '+', '?', '.', '\\', '[', ']', '(', ')', '{', '}', '^', '$',
        ];
        let might_be_regex = config.query.chars().any(|c| regex_indicators.contains(&c));

        if might_be_regex {
            for result in results {
                // Only count lines with actual matches, not context lines
                if !result.get_matching_patterns().is_empty() {
                    let pattern = &result.get_matching_patterns()[0];
                    let line = result.get_line_content();

                    // Try to compile the regex
                    let regex_result = if config.ignore_case {
                        regex::RegexBuilder::new(pattern)
                            .case_insensitive(true)
                            .build()
                    } else {
                        regex::Regex::new(pattern)
                    };

                    if let Ok(regex) = regex_result {
                        // Count all regex matches in this line
                        self.total_matches += regex.find_iter(line).count();
                    }
                }
                // If result is empty, move on to the next result
            }
        } else {
            // For regular patterns, use the existing count method
            self.total_matches += count_actual_matches(results, config.ignore_case);
        }
    }

    /// Displays the collected statistics to the console.
    pub fn display(&self) {
        println!("\n--- Search Statistics ---");
        println!("Pattern searched: '{}'", self.query);
        println!("Files searched: {}", self.files_searched);
        println!("Total lines searched: {}", self.total_lines);
        println!("Matches found: {}", self.total_matches);
        println!("Search completed in: {:.2?}", self.duration);
        println!("------------------------");
    }
}

/// Counts the total number of pattern matches across all search results.
///
/// This function performs a detailed count of all individual pattern matches,
/// including multiple occurrences of the same pattern within a single line.
///
/// # Arguments
///
/// * `results` - A slice of SearchResult objects to count matches in
/// * `ignore_case` - Whether to perform case-insensitive matching
///
/// # Returns
///
/// The total count of all pattern matches found
fn count_actual_matches(results: &[SearchResult], ignore_case: bool) -> usize {
    results
        .iter()
        .filter(|result| !result.get_matching_patterns().is_empty())
        .flat_map(|result| {
            let line = result.get_line_content();
            result.get_matching_patterns().iter().map(move |pattern| {
                let mut count = 0;
                let mut start = 0;

                let pattern_lower = if ignore_case {
                    pattern.to_lowercase()
                } else {
                    pattern.clone()
                };

                let line_lower = if ignore_case {
                    line.to_lowercase()
                } else {
                    line.to_string()
                };

                while let Some(position) = line_lower[start..].find(&pattern_lower) {
                    count += 1;
                    start += position + pattern_lower.len();
                    if start >= line_lower.len() {
                        break;
                    }
                }
                count
            })
        })
        .sum()
}
