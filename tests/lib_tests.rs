use rust_utils::{get_argv, parse_args, path_error};

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
