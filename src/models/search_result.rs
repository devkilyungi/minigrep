use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone)]
pub struct SearchResult {
    line_number: usize,
    line_content: String,
}

impl SearchResult {
    pub fn new(line_number: usize, line_content: String) -> Self {
        Self {
            line_number,
            line_content,
        }
    }
}

impl SearchResult {
    pub fn display(&self, query: &str, ignore_case: bool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut highlight_spec = ColorSpec::new();
        highlight_spec.set_fg(Some(Color::Cyan)).set_bold(true);

        // Print the line number
        print!("Line {}: ", self.line_number + 1);

        // For case-insensitive search, find all matches first
        let matches = if ignore_case {
            let query_lower = query.to_lowercase();
            let content_lower = self.line_content.to_lowercase();

            let mut indices = Vec::new();
            let mut start = 0;
            while let Some(pos) = content_lower[start..].find(&query_lower) {
                let absolute_pos = start + pos;
                indices.push((absolute_pos, absolute_pos + query_lower.len()));
                start = absolute_pos + 1;
            }
            indices
        } else {
            self.line_content
                .match_indices(query)
                .map(|(start, part)| (start, start + part.len()))
                .collect()
        };

        // Print with highlighting
        let mut last_index = 0;
        for (start, end) in matches {
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
        let _ = stdout.write_all(self.line_content[last_index..].as_ref());
        println!();
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number + 1
    }

    pub fn get_line_content(&self) -> &str {
        &self.line_content
    }
}
