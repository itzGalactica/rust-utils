use colored::Colorize;
use std::env::args;

/// Prints a path-related error message in red to stderr.
/// The message is printed in the format "{path}: {error}"
///
/// # Example
/// ```
/// use rust_utils::path_error;
///
/// let path = String::from("/home/user/some-file");
/// path_error(&path, "Failed to read");
/// ```
/// prints `/home/user/some-file: Failed to read`
pub fn path_error(path: &str, message: &str) {
    eprintln!(
        "{}{}{}",
        path.red(),
        ": ".red(),
        message.red()
    )
}

/// Collects the argument vector (containing all command-line agrs)
/// to a Vec<String>.
pub fn get_argv() -> Vec<String> {
    let argv = args().collect();
    argv
}

/// Command-line argument parser.
/// The argument vector should be passed as an &Vec<String>, and this
/// should contain the space-seperated command-line arguments exactly
/// as returned by std::env::args().collect(), including the program
/// name as the first argument.
/// The function then process each long and short option, returning the
/// name of each in a new Vec<String>.
///
/// # Example:
/// If the function is used correctly, and the program is run with:
/// `cli-app file1.txt file2.txt -abc --some-long-arg`
/// This returns `["a", "b", "c", "some-long-arg"]`
pub fn parse_args(args: &Vec<String>) -> Vec<String> {
    let mut opts: Vec<String> = Vec::new();

    for arg in args.iter().skip(1) {

        if arg.starts_with("--") {
            // Long option, add the whole thing to the vector after the
            // first two characters
            opts.push(arg[2..].to_string());
        }

        else if arg.starts_with("-") {
            // Short options, push each char after the first as a new
            // String object
            for char in arg.chars().skip(1) {
                opts.push(char.to_string());
            }
        }
    }

    opts
}

#[cfg(test)]
mod tests {
    use super::{get_argv, parse_args, path_error};

    #[test]
    fn parse_args_returns_empty_for_program_only() {
        let args = vec!["cli-app".to_string()];
        let parsed = parse_args(&args);

        assert!(parsed.is_empty());
    }

    #[test]
    fn parse_args_parses_combined_short_options() {
        let args = vec!["cli-app".to_string(), "-abc".to_string()];
        let parsed = parse_args(&args);

        assert_eq!(parsed, vec!["a", "b", "c"]);
    }

    #[test]
    fn parse_args_parses_long_options() {
        let args = vec![
            "cli-app".to_string(),
            "--verbose".to_string(),
            "--dry-run".to_string(),
        ];
        let parsed = parse_args(&args);

        assert_eq!(parsed, vec!["verbose", "dry-run"]);
    }

    #[test]
    fn parse_args_ignores_positionals_and_keeps_option_order() {
        let args = vec![
            "cli-app".to_string(),
            "input.txt".to_string(),
            "-xz".to_string(),
            "output.txt".to_string(),
            "--force".to_string(),
        ];
        let parsed = parse_args(&args);

        assert_eq!(parsed, vec!["x", "z", "force"]);
    }

    #[test]
    fn path_error_does_not_panic_with_normal_input() {
        path_error("/some/path/file.txt", "No such file or directory");
    }

    #[test]
    fn path_error_does_not_panic_with_empty_strings() {
        path_error("", "");
    }

    #[test]
    fn get_argv_returns_at_least_one_element() {
        // The first element is always the program name
        let argv = get_argv();
        assert!(!argv.is_empty());
    }
}
