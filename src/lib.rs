use models::{Config, SearchResult};
use std::{collections::HashSet, error, fs};

pub mod models;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let file_1 = fs::read_to_string(&config.file_path_1)?;
    let file_2 = if config.file_path_2.is_empty() {
        None
    } else {
        Some(fs::read_to_string(&config.file_path_2)?)
    };

    // Helper closure to print results for any file
    let print_results = |file_label: &str, contents: &str| {
        let search_results = search(
            &config.query,
            contents,
            &config.context_flag,
            Some(config.context_count as usize),
            config.ignore_case,
        );

        if search_results.is_empty() {
            println!("{file_label}: No matches found.");
        } else {
            println!("Matches in {file_label}:");
            for result in search_results {
                result.display();
            }
        }
    };

    // Search file 1
    print_results(&config.file_path_1, &file_1);

    // If file 2 exists, search it too
    if let Some(file_2_contents) = file_2 {
        print_results(&config.file_path_2, &file_2_contents);
    }

    Ok(())
}

fn search(
    query: &str,
    contents: &str,
    context: &str,
    content_count: Option<usize>,
    ignore_case: bool,
) -> Vec<SearchResult> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut results = Vec::new();
    let mut line_numbers_to_include = HashSet::new();

    // Convert query to lowercase if ignore_case is true
    let query_to_use = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    for (line_number, line_content) in lines.iter().enumerate() {
        // Check if the line contains the query, handling case sensitivity
        let contains_match = if ignore_case {
            line_content.to_lowercase().contains(&query_to_use)
        } else {
            line_content.contains(&query_to_use)
        };

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
    let mut line_numbers: Vec<usize> = line_numbers_to_include.into_iter().collect();
    line_numbers.sort();

    for &line_number in &line_numbers {
        results.push(SearchResult::new(
            line_number,
            lines[line_number].to_string(),
        ));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Config;
    use pretty_assertions::assert_eq;

    #[test]
    fn build_config() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path_1, "file_path");
    }

    #[test]
    fn test_search_no_matches() {
        let query = "nonexistent";
        let contents = "Line 1\nLine 2\nLine 3";

        let results = search(query, contents, "after", Some(1), true);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_single_match_no_context() {
        let query = "Line 2";
        let contents = "Line 1\nLine 2\nLine 3";

        let results = search(query, contents, "", None, true);

        // Print formatted results for inspection
        println!("Results: {:#?}, length: {}", results, results.len());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get_line_number(), 2);
        assert_eq!(results[0].get_line_content(), "Line 2");
    }
    
    #[test]
    fn test_search_case_insensitive() {
        let query = "line 2";
        let contents = "Line 1\nline 2\nLine 3";

        let results = search(query, contents, "", None, false);

        // Print formatted results for inspection
        println!("Results: {:#?}, length: {}", results, results.len());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get_line_number(), 2);
        assert_eq!(results[0].get_line_content(), "line 2");
    }

    #[test]
    fn test_search_before_only() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4";

        let results = search(query, contents, "before", Some(3), true);
        assert_eq!(results.len(), 3);

        // Sort by line number to ensure consistent order
        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 1);
        assert_eq!(results_sorted[1].get_line_number(), 2);
        assert_eq!(results_sorted[2].get_line_number(), 3);
    }

    #[test]
    fn test_search_after_only() {
        let query = "Line 2";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4";

        let results = search(query, contents, "after", Some(3), true);
        assert_eq!(results.len(), 3);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 2);
        assert_eq!(results_sorted[1].get_line_number(), 3);
        assert_eq!(results_sorted[2].get_line_number(), 4);
    }

    #[test]
    fn test_search_before_and_after() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

        let results = search(query, contents, "context", Some(1), true);
        assert_eq!(results.len(), 3);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number(), 3); // Line 3 (match)
        assert_eq!(results_sorted[2].get_line_number(), 4); // Line 4
    }

    #[test]
    fn test_search_at_beginning() {
        let query = "Line 1";
        let contents = "Line 1\nLine 2\nLine 3";

        let results = search(query, contents, "context", Some(1), true);
        assert_eq!(results.len(), 2);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 1); // Line 1 (match)
        assert_eq!(results_sorted[1].get_line_number(), 2); // Line 2
    }

    #[test]
    fn test_search_at_end() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3";

        let results = search(query, contents, "context", Some(1), true);
        assert_eq!(results.len(), 2);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number(), 3); // Line 3 (match)
    }

    #[test]
    fn test_search_multiple_matches() {
        let query = "Line";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

        let results = search(query, contents, "after", Some(1), true);

        // All lines should be included due to overlapping contexts
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_search_overlapping() {
        let query = "match";
        let contents = "Line 1\nLine 2\nmatch 3\nLine 4\nmatch 5\nLine 6";

        let results = search(query, contents, "context", Some(1), true);

        // Should include all lines from 1-6 due to overlapping contexts
        assert_eq!(results.len(), 5);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number(), 3); // match 3
        assert_eq!(results_sorted[2].get_line_number(), 4); // Line 4
        assert_eq!(results_sorted[3].get_line_number(), 5); // match 5
        assert_eq!(results_sorted[4].get_line_number(), 6); // Line 6
    }

    #[test]
    fn test_search_non_overlapping() {
        let query = "match";
        let contents = "Line 1\nmatch 2\nLine 3\nLine 4\nmatch 5\nLine 6";

        let results = search(query, contents, "context", Some(1), true);

        // Should have two separate groups: [0,1,2] and [3,4,5]
        assert_eq!(results.len(), 6);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number(), 1); // Line 1
        assert_eq!(results_sorted[1].get_line_number(), 2); // match 2
        assert_eq!(results_sorted[2].get_line_number(), 3); // Line 3
        assert_eq!(results_sorted[3].get_line_number(), 4); // Line 4
        assert_eq!(results_sorted[4].get_line_number(), 5); // match 5
        assert_eq!(results_sorted[5].get_line_number(), 6); // Line 6
    }

    #[test]
    fn test_search_large_ranges() {
        let query = "unique";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nunique line\nLine 6\nLine 7";

        let results = search(query, contents, "context", Some(10), true);

        // Should include all lines despite requesting more context than exists
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn test_search_empty_file() {
        let query = "anything";
        let contents = "";

        let results = search(query, contents, "after", Some(1), true);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_multiple_matches_same_line() {
        let query = "match";
        let contents = "match in a match line\nother line";

        let results = search(query, contents, "context", Some(1), true);

        // Should include lines 0 and 1 without duplicates
        assert_eq!(results.len(), 2);
    }
}
