use crate::models::{Config, SearchResult};
use std::time::Duration;

pub struct SearchStats {
    pub query: String,
    pub total_lines: usize,
    pub total_matches: usize,
    pub files_searched: usize,
    pub duration: Duration,
}

impl SearchStats {
    pub fn init_stats(config: &Config) -> Self {
        SearchStats {
            query: config.query.clone(),
            total_lines: 0,
            total_matches: 0,
            files_searched: 0,
            duration: Duration::default(),
        }
    }

    pub fn update_match_count(&mut self, results: &[SearchResult], config: &Config) {
        self.total_matches += count_actual_matches(results, config.ignore_case);
    }

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
