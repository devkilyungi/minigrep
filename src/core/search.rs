use std::collections::HashSet;
use crate::models::SearchResult;

pub fn search(
    query: &str,
    contents: &str,
    context: &str,
    content_count: Option<usize>,
    ignore_case: bool,
) -> Vec<SearchResult> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut line_numbers_to_include = HashSet::new();

    // Split query into patterns by pipe character
    let patterns = if query.contains('|') {
        query
            .split('|')
            .map(|pattern| pattern.trim().to_string())
            .collect::<Vec<String>>()
    } else {
        vec![query.to_string()]
    };

    // Convert patterns in query to lowercase if ignore_case is true
    let patterns_to_use = if ignore_case {
        patterns
            .iter()
            .map(|pattern| pattern.to_lowercase())
            .collect::<Vec<String>>()
    } else {
        patterns
    };

    for (line_number, line_content) in lines.iter().enumerate() {
        // Handle case sensitivity for the line content
        let line_to_check = if ignore_case {
            line_content.to_lowercase()
        } else {
            line_content.to_string()
        };

        // Check if the line contains any of the patterns
        let contains_match = patterns_to_use
            .iter()
            .any(|pattern| line_to_check.contains(pattern));

        if contains_match {
            // Include the matched line number
            line_numbers_to_include.insert(line_number);

            // Handle context based on the flag
            match context {
                "before" => {
                    if let Some(before_count) = content_count {
                        let first_line = line_number.saturating_sub(before_count);
                        for i in first_line..line_number {
                            line_numbers_to_include.insert(i);
                        }
                    }
                }
                "after" => {
                    if let Some(after_count) = content_count {
                        let next_line = line_number + 1;
                        let last_line = (line_number + after_count).min(lines.len() - 1);
                        for i in next_line..=last_line {
                            line_numbers_to_include.insert(i);
                        }
                    }
                }
                "context" => {
                    if let Some(context_count) = content_count {
                        let first_line = line_number.saturating_sub(context_count);
                        let last_line = (line_number + context_count).min(lines.len() - 1);
                        for i in first_line..=last_line {
                            line_numbers_to_include.insert(i);
                        }
                    }
                }
                _ => {
                    // Default case with no context
                    // Matched line number already included
                }
            }
        }
    }

    // Then collect the results in order
    let mut results = Vec::new();
    let mut line_numbers: Vec<usize> = line_numbers_to_include.into_iter().collect();
    line_numbers.sort();

    for &line_number in &line_numbers {
        // Store the original matching patterns for this line to use in highlighting
        let matching_patterns: Vec<String> = patterns_to_use
            .iter()
            .filter(|&pattern| {
                if ignore_case {
                    lines[line_number].to_lowercase().contains(pattern)
                } else {
                    lines[line_number].contains(pattern)
                }
            })
            .cloned()
            .collect();

        results.push(SearchResult::new(
            line_number,
            lines[line_number].to_string(),
            matching_patterns,
        ));
    }

    results
}