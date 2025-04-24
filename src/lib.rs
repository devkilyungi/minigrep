pub mod config;
pub mod core;
mod models;
mod utils;

use models::{Config, SearchStats};
use std::{error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let start_time = std::time::Instant::now();
    let mut stats = SearchStats::init_stats(&config);

    // Check file existence upfront
    if !config.recursive && !std::path::Path::new(&config.file_path_1).exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", config.file_path_1),
        )));
    }

    if !config.file_path_2.is_empty() && !std::path::Path::new(&config.file_path_2).exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", config.file_path_2),
        )));
    }

    // If recursive flag is set, get all files from directory
    if config.recursive {
        let files = utils::get_all_files_in_directory(&config.file_path_1)?;
        stats.files_searched = files.len();

        for file_path in files {
            if let Ok(contents) = fs::read_to_string(&file_path) {
                stats.total_lines += contents.lines().count();

                // Handle the Result returned by search
                match core::search(
                    &config.query,
                    &contents,
                    config.context_flag.as_str(),
                    Some(config.context_count as usize),
                    config.ignore_case,
                ) {
                    Ok(search_results) => {
                        stats.update_match_count(&search_results, &config);
                        if !search_results.is_empty() {
                            if let Some(path_str) = file_path.to_str() {
                                core::display_results(
                                    path_str,
                                    &search_results,
                                    config.ignore_case,
                                );
                            }
                        }
                    }
                    Err(e) => {
                        if let Some(path_str) = file_path.to_str() {
                            eprintln!("Error searching file {}: {}", path_str, e);
                        }
                        // Continue with next file instead of stopping the entire search
                    }
                }
            }
        }
    } else {
        let file_1 = fs::read_to_string(&config.file_path_1)?;
        stats.total_lines += file_1.lines().count();
        stats.files_searched += 1;

        let file_2 = if config.file_path_2.is_empty() {
            None
        } else {
            let content = fs::read_to_string(&config.file_path_2)?;
            stats.total_lines += content.lines().count();
            stats.files_searched += 1;
            Some(content)
        };

        // Search and display file 1
        match core::search(
            &config.query,
            &file_1,
            config.context_flag.as_str(),
            Some(config.context_count as usize),
            config.ignore_case,
        ) {
            Ok(search_results_1) => {
                stats.update_match_count(&search_results_1, &config);
                core::display_results(&config.file_path_1, &search_results_1, config.ignore_case);
            }
            Err(e) => {
                eprintln!("Error searching file {}: {}", config.file_path_1, e);
                // Don't return early, continue with file 2 if it exists
            }
        }

        // If file 2 exists, search and display it too
        if let Some(file_2_contents) = file_2 {
            match core::search(
                &config.query,
                &file_2_contents,
                config.context_flag.as_str(),
                Some(config.context_count as usize),
                config.ignore_case,
            ) {
                Ok(search_results_2) => {
                    stats.update_match_count(&search_results_2, &config);
                    core::display_results(
                        &config.file_path_2,
                        &search_results_2,
                        config.ignore_case,
                    );
                }
                Err(e) => {
                    eprintln!("Error searching file {}: {}", config.file_path_2, e);
                }
            }
        }
    }

    // Print stats if requested
    if config.show_stats {
        stats.duration = start_time.elapsed();
        stats.display();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{config::parse_args, core::search, models::ContextFlag};
    use pretty_assertions::assert_eq;

    #[test]
    fn build_config() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];

        let config = parse_args(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path_1, "file_path");
    }

    #[test]
    fn test_search_no_matches() {
        let query = "nonexistent";
        let contents = "Line 1\nLine 2\nLine 3";
        let context_flag = ContextFlag::After.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_single_match_no_context() {
        let query = "Line 2";
        let contents = "Line 1\nLine 2\nLine 3";

        let results = search(query, contents, "", None, true).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get_line_number() + 1, 2);
        assert_eq!(results[0].get_line_content(), "Line 2");
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "line 2";
        let contents = "Line 1\nline 2\nLine 3";

        let results = search(query, contents, "", None, false).unwrap();

        // Print formatted results for inspection
        println!("Results: {:#?}, length: {}", results, results.len());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get_line_number() + 1, 2);
        assert_eq!(results[0].get_line_content(), "line 2");
    }

    #[test]
    fn test_search_before_only() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4";
        let context_flag = ContextFlag::Before.as_str();

        let results = search(query, contents, context_flag, Some(3), true).unwrap();
        assert_eq!(results.len(), 3);

        // Sort by line number to ensure consistent order
        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 1);
        assert_eq!(results_sorted[1].get_line_number() + 1, 2);
        assert_eq!(results_sorted[2].get_line_number() + 1, 3);
    }

    #[test]
    fn test_search_after_only() {
        let query = "Line 2";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4";
        let context_flag = ContextFlag::After.as_str();

        let results = search(query, contents, context_flag, Some(3), true).unwrap();
        assert_eq!(results.len(), 3);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 2);
        assert_eq!(results_sorted[1].get_line_number() + 1, 3);
        assert_eq!(results_sorted[2].get_line_number() + 1, 4);
    }

    #[test]
    fn test_search_before_and_after() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();
        assert_eq!(results.len(), 3);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number() + 1, 3); // Line 3 (match)
        assert_eq!(results_sorted[2].get_line_number() + 1, 4); // Line 4
    }

    #[test]
    fn test_search_at_beginning() {
        let query = "Line 1";
        let contents = "Line 1\nLine 2\nLine 3";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();
        assert_eq!(results.len(), 2);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 1); // Line 1 (match)
        assert_eq!(results_sorted[1].get_line_number() + 1, 2); // Line 2
    }

    #[test]
    fn test_search_at_end() {
        let query = "Line 3";
        let contents = "Line 1\nLine 2\nLine 3";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();
        assert_eq!(results.len(), 2);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number() + 1, 3); // Line 3 (match)
    }

    #[test]
    fn test_search_multiple_matches() {
        let query = "Line";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let context_flag = ContextFlag::After.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();

        // All lines should be included due to overlapping contexts
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_search_overlapping() {
        let query = "match";
        let contents = "Line 1\nLine 2\nmatch 3\nLine 4\nmatch 5\nLine 6";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();

        // Should include all lines from 1-6 due to overlapping contexts
        assert_eq!(results.len(), 5);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 2); // Line 2
        assert_eq!(results_sorted[1].get_line_number() + 1, 3); // match 3
        assert_eq!(results_sorted[2].get_line_number() + 1, 4); // Line 4
        assert_eq!(results_sorted[3].get_line_number() + 1, 5); // match 5
        assert_eq!(results_sorted[4].get_line_number() + 1, 6); // Line 6
    }

    #[test]
    fn test_search_non_overlapping() {
        let query = "match";
        let contents = "Line 1\nmatch 2\nLine 3\nLine 4\nmatch 5\nLine 6";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();

        // Should have two separate groups: [0,1,2] and [3,4,5]
        assert_eq!(results.len(), 6);

        let mut results_sorted = results.clone();
        results_sorted.sort_by_key(|r| r.get_line_number());

        assert_eq!(results_sorted[0].get_line_number() + 1, 1); // Line 1
        assert_eq!(results_sorted[1].get_line_number() + 1, 2); // match 2
        assert_eq!(results_sorted[2].get_line_number() + 1, 3); // Line 3
        assert_eq!(results_sorted[3].get_line_number() + 1, 4); // Line 4
        assert_eq!(results_sorted[4].get_line_number() + 1, 5); // match 5
        assert_eq!(results_sorted[5].get_line_number() + 1, 6); // Line 6
    }

    #[test]
    fn test_search_large_ranges() {
        let query = "unique";
        let contents = "Line 1\nLine 2\nLine 3\nLine 4\nunique line\nLine 6\nLine 7";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(10), true).unwrap();

        // Should include all lines despite requesting more context than exists
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn test_search_empty_file() {
        let query = "anything";
        let contents = "";
        let context_flag = ContextFlag::After.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_multiple_matches_same_line() {
        let query = "match";
        let contents = "match in a match line\nother line";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();

        // Should include lines 0 and 1 without duplicates
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_with_regex() {
        let query = "Line \\d"; // Regex pattern matching "Line" followed by a digit
        let contents = "Line 1\nLine 2\nLine 3\nNo match";

        let results = search(query, contents, "", None, true).unwrap();

        assert_eq!(results.len(), 3);
        let line_numbers: Vec<usize> = results.iter().map(|r| r.get_line_number()).collect();
        assert!(line_numbers.contains(&0)); // Line 1
        assert!(line_numbers.contains(&1)); // Line 2
        assert!(line_numbers.contains(&2)); // Line 3
    }

    #[test]
    fn test_regex_case_sensitivity() {
        let query = "[Ll]ine \\d";
        let contents = "Line 1\nline 2\nLINE 3";

        // Case-sensitive search should match only the first line
        let results_sensitive = search(query, contents, "", None, false).unwrap();
        assert_eq!(results_sensitive.len(), 2);

        // Case-insensitive search should match all three lines
        let results_insensitive = search(query, contents, "", None, true).unwrap();
        assert_eq!(results_insensitive.len(), 3);
    }

    #[test]
    fn test_invalid_regex() {
        let query = "Line ["; // Invalid regex pattern (unclosed character class)
        let contents = "Line 1\nLine 2";

        let result = search(query, contents, "", None, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_regex_with_context() {
        let query = "Line \\d";
        let contents = "Header\nLine 1\nMiddle\nLine 3\nFooter";
        let context_flag = ContextFlag::Context.as_str();

        let results = search(query, contents, context_flag, Some(1), true).unwrap();

        // Should include: Header, Line 1, Middle, Line 3, Footer (all 5 lines)
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_pipe_chars_in_regex() {
        let query = "Line|Header";
        let contents = "Header\nLine 1\nMiddle\nLine 3\nFooter";

        let results = search(query, contents, "", None, true).unwrap();

        // Should match lines with "Line" or "Header"
        assert_eq!(results.len(), 3);
    }
}
