use std::error::Error;
use std::fs;

pub mod models;

pub fn run(config: models::Config) -> Result<(), Box<dyn Error>> {
    let file_1 = fs::read_to_string(&config.file_path_1)?;
    let file_2 = if config.file_path_2.is_empty() {
        None
    } else {
        Some(fs::read_to_string(&config.file_path_2)?)
    };

    // Helper closure to print results for any file
    let print_results = |file_label: &str, contents: &str| {
        let search_results = if config.ignore_case {
            search_case_insensitive(&config.query, contents)
        } else {
            search(&config.query, contents)
        };

        if search_results.is_empty() {
            println!("{file_label}: No matches found.");
        } else {
            println!("Matches in {file_label}:");
            for res in search_results {
                println!("Line {}: {}", res.get_line_number(), res.get_line_content());
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

fn search(query: &str, contents: &str) -> Vec<models::SearchResult> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line_content)| line_content.contains(query))
        .map(|(line_number, line_content)| {
            models::SearchResult::new(line_number, line_content.to_string())
        })
        .collect()
}

fn search_case_insensitive(query: &str, contents: &str) -> Vec<models::SearchResult> {
    let query = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line_content)| line_content.to_lowercase().contains(&query))
        .map(|(line_number, line_content)| {
            models::SearchResult::new(line_number, line_content.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let search_results: Vec<models::SearchResult> = search(query, contents);
        let formatted_results = search_results
            .iter()
            .map(|search_result| {
                format!(
                    "Line {}: {}",
                    search_result.get_line_number(),
                    search_result.get_line_content()
                )
            })
            .collect::<Vec<String>>();

        assert_eq!("Line 1: Rust:", formatted_results[0]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";

        let search_results: Vec<models::SearchResult> = search_case_insensitive(query, contents);
        let formatted_results = search_results
            .iter()
            .map(|search_result| {
                format!(
                    "Line {}: {}",
                    search_result.get_line_number(),
                    search_result.get_line_content()
                )
            })
            .collect::<Vec<String>>();

        assert_eq!("Line 1: Rust:", formatted_results[0]);
    }

    #[test]
    fn build_config() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];

        let config = models::Config::build(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path_1, "file_path");
    }
}
