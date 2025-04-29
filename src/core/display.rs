//! Functionality for displaying search results with formatting and highlighting.

use crate::models::SearchResult;
use regex::Regex;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Displays search results for a specific file with highlighting.
///
/// # Arguments
///
/// * `file_label` - Name or path of the file containing the matches
/// * `results` - Search results to display
/// * `ignore_case` - Whether the search was case-insensitive
pub fn display_results(file_label: &str, results: &[SearchResult], ignore_case: bool) {
    if results.is_empty() {
        println!("{file_label}: No matches found.");
    } else {
        println!("Matches in {file_label}:");
        for result in results {
            display_search_result(result, ignore_case);
        }
    }
}

/// Displays a single search result with highlighted matches.
///
/// # Arguments
///
/// * `search_result` - The search result to display
/// * `ignore_case` - Whether the search was case-insensitive
fn display_search_result(search_result: &SearchResult, ignore_case: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut highlight_spec = ColorSpec::new();
    highlight_spec.set_fg(Some(Color::Cyan)).set_bold(true);

    // Print the line number
    print!("Line {}: ", search_result.get_line_number() + 1);

    // Check if there are any matching patterns
    let matching_patterns = search_result.get_matching_patterns();
    if matching_patterns.is_empty() {
        // No patterns to highlight, just print the line
        println!("{}", search_result.get_line_content());
        return;
    }

    // Check if the pattern might be a regex
    let pattern = &matching_patterns[0];
    let regex_indicators = [
        '*', '+', '?', '.', '\\', '[', ']', '(', ')', '{', '}', '^', '$',
    ];
    let might_be_regex = pattern.chars().any(|c| regex_indicators.contains(&c));

    if might_be_regex {
        // Try to compile the regex for highlighting
        let regex_result = if ignore_case {
            regex::RegexBuilder::new(pattern)
                .case_insensitive(true)
                .build()
        } else {
            Regex::new(pattern)
        };

        if let Ok(regex) = regex_result {
            // Highlight regex matches
            let line = &search_result.get_line_content();
            let mut last_match_end = 0;

            // Find all matches and highlight them
            for captures in regex.captures_iter(line) {
                let m = captures.get(0).unwrap();

                // Print text before match
                let before_match = &line[last_match_end..m.start()];
                let _ = write!(&mut stdout, "{}", before_match);

                // Print highlighted match
                let _ = stdout.set_color(&highlight_spec);
                let match_text = &line[m.start()..m.end()];
                let _ = write!(&mut stdout, "{}", match_text);
                let _ = stdout.reset();

                last_match_end = m.end();
            }

            // Print remaining text
            if last_match_end < line.len() {
                let remaining = &line[last_match_end..];
                let _ = write!(&mut stdout, "{}", remaining);
            }

            println!();
        } else {
            // Fall back to normal display if regex is invalid
            println!("{}", search_result.get_line_content());
        }
    } else {
        // Find all matches for all patterns
        let mut matches = Vec::new();

        for pattern in search_result.get_matching_patterns() {
            let pattern_matches = if ignore_case {
                let pattern_lower = pattern.to_lowercase();
                let content_lower = search_result.get_line_content().to_lowercase();

                let mut indices = Vec::new();
                let mut start = 0;
                while let Some(position) = content_lower[start..].find(&pattern_lower) {
                    let absolute_position = start + position;
                    // (start of match, end of match)
                    indices.push((absolute_position, absolute_position + pattern_lower.len()));
                    start = absolute_position + 1;
                }
                indices
            } else {
                search_result
                    .get_line_content()
                    .match_indices(pattern)
                    .map(|(start, part)| (start, start + part.len()))
                    .collect::<Vec<_>>()
            };

            matches.extend(pattern_matches);
        }

        // Sort matches by start position
        matches.sort_by_key(|&(start, _)| start);

        // Merge overlapping matches
        let mut merged_matches = Vec::new();
        for (start, end) in matches {
            if let Some((_, prev_end)) = merged_matches.last_mut() {
                // If this match overlaps with previous, merge them
                if start <= *prev_end {
                    *prev_end = end.max(*prev_end);
                } else {
                    merged_matches.push((start, end));
                }
            } else {
                merged_matches.push((start, end));
            }
        }

        // Print with highlighting
        let mut last_index = 0;
        for (start, end) in merged_matches {
            // Text before match
            let before_match = &search_result.get_line_content()[last_index..start];
            let _ = write!(&mut stdout, "{}", before_match);

            // Highlighted match
            let _ = stdout.set_color(&highlight_spec);
            let match_text = &search_result.get_line_content()[start..end];
            let _ = write!(&mut stdout, "{}", match_text);
            let _ = stdout.reset();

            last_index = end;
        }

        // Remaining text
        let remaining = &search_result.get_line_content()[last_index..];
        let _ = write!(&mut stdout, "{}", remaining);
        println!();
    }
}

/// Prints help information about the minigrep tool.
pub fn print_help() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Create color specs
    let mut green_bold = ColorSpec::new();
    green_bold.set_fg(Some(Color::Green)).set_bold(true);

    let mut cyan = ColorSpec::new();
    cyan.set_fg(Some(Color::Cyan));

    // Helper function to print a section header
    fn print_section(stdout: &mut StandardStream, color_spec: &ColorSpec, text: &str) {
        let _ = stdout.set_color(color_spec);
        let _ = writeln!(stdout, "{}", text);
        let _ = stdout.reset();
    }

    // Helper function to print an option and its description with aligned spacing
    fn print_option(
        stdout: &mut StandardStream,
        color_spec: &ColorSpec,
        option: &str,
        description: &str,
        section: &str,
    ) {
        let _ = write!(stdout, "  "); // Indent options
        let _ = stdout.set_color(color_spec);
        let _ = write!(stdout, "{}", option);
        let _ = stdout.reset();

        // Calculate padding needed based on section type
        let column_width = match section {
            "EXAMPLES" | "REGEX EXAMPLES" => 40, // Examples need more space
            "CONTEXT OPTIONS" | "OUTPUT OPTIONS" | "DIRECTORY OPTIONS" => 25, // Options need more space
            "SEARCH OPTIONS" | "ARGUMENTS" => 20,                             // Short options
            "OTHER OPTIONS" => 20,                                            // Short options
            "ENVIRONMENT" => 15,                                              // Short options
            "EXIT CODES" => 10,                                               // Very short options
            _ => 30, // Default for other sections
        };

        let padding_length = if option.len() >= column_width {
            2
        } else {
            column_width - option.len()
        };
        let padding = " ".repeat(padding_length);
        let _ = writeln!(stdout, "{}{}", padding, description);
    }

    // Title
    let _ = writeln!(
        &mut stdout,
        "minigrep v{} - Search for patterns in files",
        env!("CARGO_PKG_VERSION")
    );
    let _ = writeln!(&mut stdout);

    // Usage
    print_section(&mut stdout, &green_bold, "USAGE:");
    let _ = writeln!(
        &mut stdout,
        "    minigrep PATTERN FILENAME [SECOND_FILENAME] [OPTIONS]"
    );
    let _ = writeln!(&mut stdout);

    // Arguments
    print_section(&mut stdout, &green_bold, "ARGUMENTS:");
    print_option(
        &mut stdout,
        &cyan,
        "PATTERN",
        "Text or regex pattern to search for",
        "ARGUMENTS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "FILENAME",
        "File or directory to search in",
        "ARGUMENTS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "[SECOND_FILENAME]",
        "Optional second file to search in",
        "ARGUMENTS",
    );
    let _ = writeln!(&mut stdout);

    // Search options
    print_section(&mut stdout, &green_bold, "SEARCH OPTIONS:");
    print_option(
        &mut stdout,
        &cyan,
        "-ic",
        "Ignore case when searching",
        "SEARCH OPTIONS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "-cs",
        "Force case-sensitive search (overrides IGNORE_CASE env)",
        "SEARCH OPTIONS",
    );
    let _ = writeln!(&mut stdout);

    // Context options
    print_section(&mut stdout, &green_bold, "CONTEXT OPTIONS:");
    print_option(
        &mut stdout,
        &cyan,
        "--before N, --b N",
        "Show N lines before each match",
        "CONTEXT OPTIONS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "--after N, --a N",
        "Show N lines after each match",
        "CONTEXT OPTIONS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "--context N, --c N",
        "Show N lines before and after each match",
        "CONTEXT OPTIONS",
    );
    let _ = writeln!(&mut stdout);

    // Output options
    print_section(&mut stdout, &green_bold, "OUTPUT OPTIONS:");
    print_option(
        &mut stdout,
        &cyan,
        "--stats, --s",
        "Display search statistics",
        "OUTPUT OPTIONS",
    );
    let _ = writeln!(&mut stdout);

    // Directory options
    print_section(&mut stdout, &green_bold, "DIRECTORY OPTIONS:");
    print_option(
        &mut stdout,
        &cyan,
        "--recursive, --r",
        "Recursively search through all files in a directory",
        "DIRECTORY OPTIONS",
    );
    let _ = writeln!(&mut stdout);

    // Other options
    print_section(&mut stdout, &green_bold, "OTHER OPTIONS:");
    print_option(
        &mut stdout,
        &cyan,
        "--help, -h",
        "Display this help message",
        "OTHER OPTIONS",
    );
    print_option(
        &mut stdout,
        &cyan,
        "--version, -v",
        "Display version information",
        "OTHER OPTIONS",
    );
    let _ = writeln!(&mut stdout);

    // Examples
    print_section(&mut stdout, &green_bold, "EXAMPLES:");
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt",
        "Basic search",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt -ic",
        "Case-insensitive search",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt --stats",
        "Show search statistics",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt --context 2",
        "Show context around matches",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt sunrise.txt",
        "Search in multiple files",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to directory --recursive",
        "Search recursively in directory",
        "EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep to poem.txt -ic --stats",
        "Combine multiple options",
        "EXAMPLES",
    );
    let _ = writeln!(&mut stdout);

    // Regex examples
    print_section(&mut stdout, &green_bold, "REGEX EXAMPLES:");
    print_option(
        &mut stdout,
        &cyan,
        "minigrep \"\\bw\\w+\" poem.txt",
        "Find all words starting with 'w'",
        "REGEX EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep \"s.n\" sunrise.txt",
        "Match any character between 's' and 'n'",
        "REGEX EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep \"\\w+ing\\b\" poem.txt -ic",
        "Find words ending in 'ing' (case insensitive)",
        "REGEX EXAMPLES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "minigrep \"test|assert\" dir/ --r",
        "Find 'test' or 'assert' in directory",
        "REGEX EXAMPLES",
    );
    let _ = writeln!(&mut stdout);

    // Environment
    print_section(&mut stdout, &green_bold, "ENVIRONMENT:");
    print_option(
        &mut stdout,
        &cyan,
        "IGNORE_CASE",
        "Set to any value to enable case-insensitive search by default",
        "ENVIRONMENT",
    );
    let _ = writeln!(&mut stdout);

    // Exit codes
    print_section(&mut stdout, &green_bold, "EXIT CODES:");
    print_option(
        &mut stdout,
        &cyan,
        "0",
        "Successful execution",
        "EXIT CODES",
    );
    print_option(
        &mut stdout,
        &cyan,
        "1",
        "Error occurred (invalid arguments, file not found, etc.)",
        "EXIT CODES",
    );
}
