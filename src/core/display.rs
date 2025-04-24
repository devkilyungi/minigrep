use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::models::SearchResult;

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
            search_result.get_line_content()
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