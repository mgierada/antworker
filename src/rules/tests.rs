use crate::rules::define::FilterRules;

#[test]
fn test_filter_rules_is_empty_with_empty_senders() {
    let rules = FilterRules {
        allowed_senders: vec![],
        timeframe: None,
    };
    assert!(rules.is_empty());
}

#[test]
fn test_filter_rules_is_empty_with_senders() {
    let rules = FilterRules {
        allowed_senders: vec!["test@example.com".to_string()],
        timeframe: None,
    };
    assert!(!rules.is_empty());
}

#[test]
fn test_filter_rules_is_empty_with_multiple_senders() {
    let rules = FilterRules {
        allowed_senders: vec![
            "test1@example.com".to_string(),
            "test2@example.com".to_string(),
        ],
        timeframe: None,
    };
    assert!(!rules.is_empty());
}

#[test]
fn test_filter_rules_with_timeframe() {
    let rules = FilterRules {
        allowed_senders: vec!["test@example.com".to_string()],
        timeframe: Some((2024, 11)),
    };
    assert!(!rules.is_empty());
    assert_eq!(rules.timeframe, Some((2024, 11)));
}

#[test]
fn test_filter_rules_no_timeframe() {
    let rules = FilterRules {
        allowed_senders: vec!["test@example.com".to_string()],
        timeframe: None,
    };
    assert!(rules.timeframe.is_none());
}
