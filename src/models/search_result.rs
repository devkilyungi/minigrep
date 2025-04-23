use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone)]
pub struct SearchResult {
    line_number: usize,
    line_content: String,
    matching_patterns: Vec<String>,
}

impl SearchResult {
    pub fn new(line_number: usize, line_content: String, matching_patterns: Vec<String>) -> Self {
        Self {
            line_number,
            line_content,
            matching_patterns,
        }
    }

    pub fn get_matching_patterns(&self) -> &[String] {
        &self.matching_patterns
    }
}

impl SearchResult {
    pub fn display(&self, ignore_case: bool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut highlight_spec = ColorSpec::new();
        highlight_spec.set_fg(Some(Color::Cyan)).set_bold(true);

        // Print the line number
        print!("Line {}: ", self.line_number + 1);

        // Find all matches for all patterns
        let mut matches = Vec::new();

        for pattern in &self.matching_patterns {
            let pattern_matches = if ignore_case {
                let pattern_lower = pattern.to_lowercase();
                let content_lower = self.line_content.to_lowercase();

                let mut indices = Vec::new();
                let mut start = 0;
                while let Some(pos) = content_lower[start..].find(&pattern_lower) {
                    let absolute_pos = start + pos;
                    indices.push((absolute_pos, absolute_pos + pattern_lower.len()));
                    start = absolute_pos + 1;
                }
                indices
            } else {
                self.line_content
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
            let before_match = &self.line_content[last_index..start];
            let _ = write!(&mut stdout, "{}", before_match);

            // Highlighted match
            let _ = stdout.set_color(&highlight_spec);
            let match_text = &self.line_content[start..end];
            let _ = write!(&mut stdout, "{}", match_text);
            let _ = stdout.reset();

            last_index = end;
        }

        // Remaining text
        let remaining = &self.line_content[last_index..];
        let _ = write!(&mut stdout, "{}", remaining);
        println!();
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number + 1
    }

    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
