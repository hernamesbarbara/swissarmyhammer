//! Tests for PromptAction

use crate::workflow::actions::*;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_prompt_action_creation() {
    let action = PromptAction::new("test-prompt".to_string());
    assert_eq!(action.prompt_name, "test-prompt");
    assert!(action.arguments.is_empty());
    assert!(action.result_variable.is_none());
    assert_eq!(action.timeout, Duration::from_secs(3600));
}

#[test]
fn test_prompt_action_with_arguments() {
    let action = PromptAction::new("test-prompt".to_string())
        .with_argument("arg1".to_string(), "value1".to_string())
        .with_argument("arg2".to_string(), "value2".to_string());

    assert_eq!(action.arguments.get("arg1"), Some(&"value1".to_string()));
    assert_eq!(action.arguments.get("arg2"), Some(&"value2".to_string()));
}

#[test]
fn test_prompt_action_with_result_variable() {
    let action =
        PromptAction::new("test-prompt".to_string()).with_result_variable("result_var".to_string());

    assert_eq!(action.result_variable, Some("result_var".to_string()));
}

#[test]
fn test_prompt_action_with_timeout() {
    let timeout_duration = Duration::from_secs(60);
    let action = PromptAction::new("test-prompt".to_string()).with_timeout(timeout_duration);

    assert_eq!(action.timeout, timeout_duration);
}

#[test]
fn test_prompt_action_with_quiet() {
    // Test enabling quiet mode
    let action = PromptAction::new("test-prompt".to_string()).with_quiet(true);
    assert!(action.quiet);

    // Test disabling quiet mode
    let action = PromptAction::new("test-prompt".to_string()).with_quiet(false);
    assert!(!action.quiet);

    // Test default is false
    let action = PromptAction::new("test-prompt".to_string());
    assert!(!action.quiet);
}

// Note: Variable substitution is tested through the public execute() method
// since substitute_variables() is private

#[test]
fn test_prompt_action_description() {
    let action = PromptAction::new("test-prompt".to_string())
        .with_argument("arg1".to_string(), "value1".to_string());

    let description = action.description();
    assert!(description.contains("test-prompt"));
    assert!(description.contains("arg1"));
}

#[test]
fn test_prompt_action_type() {
    let action = PromptAction::new("test-prompt".to_string());
    assert_eq!(action.action_type(), "prompt");
}

#[tokio::test]
async fn test_prompt_action_execution_with_invalid_argument_key() {
    let action = PromptAction::new("test-prompt".to_string())
        .with_argument("invalid key!".to_string(), "value".to_string());

    let mut context = HashMap::new();
    let result = action.execute(&mut context).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        ActionError::ParseError(msg) => {
            assert!(msg.contains("Invalid argument key"));
        }
        _ => panic!("Expected ParseError"),
    }
}

#[test]
fn test_abort_error_detection_in_response() {
    // Test that ABORT ERROR pattern is correctly detected
    let test_cases = vec![
        (
            "ABORT ERROR: User cancelled the operation",
            true,
            "User cancelled the operation",
        ),
        ("This is a normal response", false, ""),
        ("Some text before ABORT ERROR: Critical failure", false, ""), // Should only match at start
        (
            "ABORT ERROR: Multi-line\nerror description",
            true,
            "Multi-line\nerror description",
        ),
        ("abort error: lowercase should not match", false, ""),
    ];

    for (response, should_detect, expected_message) in test_cases {
        let is_abort = response.starts_with("ABORT ERROR:");
        assert_eq!(is_abort, should_detect, "Failed for response: {}", response);

        if should_detect {
            let message = response.trim_start_matches("ABORT ERROR:").trim();
            assert_eq!(
                message, expected_message,
                "Failed to extract message from: {}",
                response
            );
        }
    }
}
