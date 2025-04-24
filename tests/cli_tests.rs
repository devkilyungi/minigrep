#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;

    fn strip_ansi_color_codes(s: &str) -> String {
        let re = regex::Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
        re.replace_all(s, "").to_string()
    }

    #[test]
    fn test_cli_basic_search() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        let output = cmd
            .arg("the")
            .arg("tests/fixtures/poem.txt")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        let clean_stdout = strip_ansi_color_codes(&stdout);

        assert!(clean_stdout.contains("Line 3: Then there's a pair of us - don't tell!"));
        assert!(clean_stdout.contains("Line 8: To tell your name the livelong day"));
    }

    #[test]
    fn test_case_insensitive_search() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        // Run with case-insensitive flag
        let output = cmd
            .arg("THE")
            .arg("tests/fixtures/poem.txt")
            .arg("-ic")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        let clean_stdout = strip_ansi_color_codes(&stdout);

        assert!(clean_stdout.contains("Line 3: Then there's a pair of us - don't tell!"));
    }

    #[test]
    fn test_context_flag() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        // Run with context flag
        let output = cmd
            .arg("frog")
            .arg("tests/fixtures/poem.txt")
            .arg("--context")
            .arg("1")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        let clean_stdout = strip_ansi_color_codes(&stdout);

        assert!(clean_stdout.contains("Line 6: How dreary to be somebody!"));
        assert!(clean_stdout.contains("Line 7: How public, like a frog"));
        assert!(clean_stdout.contains("Line 8: To tell your name the livelong day"));
    }

    #[test]
    fn test_stats_flag() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        // Run with stats flag
        let output = cmd
            .arg("to")
            .arg("tests/fixtures/poem.txt")
            .arg("--stats")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        let clean_stdout = strip_ansi_color_codes(&stdout);

        assert!(clean_stdout.contains("Line 2: Are you nobody, too?"));
        assert!(clean_stdout.contains("Line 6: How dreary to be somebody!"));
        assert!(clean_stdout.contains("--- Search Statistics ---"));
        assert!(clean_stdout.contains("Pattern searched: 'to'"));
        assert!(clean_stdout.contains("Files searched: 1"));
        assert!(clean_stdout.contains("Total lines searched: 9"));
        assert!(clean_stdout.contains("Matches found: 2"));
    }

    #[test]
    fn test_invalid_args() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        // Not enough arguments
        let output = cmd
            .arg("too-few")
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8(output.stderr).unwrap();

        assert!(stderr.contains("Problem parsing arguments: Not enough arguments"));
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn test_file_not_found() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();

        // File doesn't exist
        let output = cmd
            .arg("query")
            .arg("nonexistent-file.txt")
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8(output.stderr).unwrap();

        assert!(stderr.contains("Application error:"));
        assert!(stderr.contains("No such file or directory"));
        assert_eq!(output.status.code(), Some(1));
    }
}
