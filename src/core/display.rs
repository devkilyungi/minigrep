use crate::models::SearchResult;
use regex::Regex;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

                // Print text before match (safer approach)
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
                while let Some(pos) = content_lower[start..].find(&pattern_lower) {
                    let absolute_pos = start + pos;
                    indices.push((absolute_pos, absolute_pos + pattern_lower.len()));
                    start = absolute_pos + 1;
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

// In display.rs, update print_help function:
pub fn print_help() {
    println!(
        "minigrep v{} - Search for patterns in files",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("USAGE:");
    println!("    minigrep PATTERN FILENAME [SECOND_FILENAME] [OPTIONS]");
    println!();
    println!("ARGUMENTS:");
    println!("    PATTERN               Text or regex pattern to search for");
    println!("    FILENAME              File or directory to search in");
    println!("    [SECOND_FILENAME]     Optional second file to search in");
    println!();
    println!("SEARCH OPTIONS:");
    println!("    -ic                   Ignore case when searching");
    println!("    -cs                   Force case-sensitive search (overrides IGNORE_CASE env)");
    println!();
    println!("CONTEXT OPTIONS:");
    println!("    --before N, --b N     Show N lines before each match");
    println!("    --after N, --a N      Show N lines after each match");
    println!("    --context N, --c N    Show N lines before and after each match");
    println!();
    println!("OUTPUT OPTIONS:");
    println!("    --stats, --s          Display search statistics");
    println!();
    println!("DIRECTORY OPTIONS:");
    println!("    --recursive, --r      Recursively search through all files in a directory");
    println!();
    println!("OTHER OPTIONS:");
    println!("    --help, -h            Display this help message");
    println!("    --version, -v         Display version information");
    println!();
    println!("REGEX EXAMPLES:");
    println!("    minigrep \"\\bw\\w+\" poem.txt     Find all words starting with 'w'");
    println!("    minigrep \"s.n\" sunrise.txt     Match any character between 's' and 'n'");
    println!("    minigrep \"\\w+ing\\b\" -ic       Find words ending in 'ing' (case insensitive)");
    println!();
    println!("ENVIRONMENT:");
    println!(
        "    IGNORE_CASE           Set to any value to enable case-insensitive search by default"
    );
}
