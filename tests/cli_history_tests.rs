#[cfg(test)]
mod tests {
    use aumlang::cli::cli_history::{HISTORY_COMMAND_MAX_SIZE, ReplHistory};

    #[test]
    fn push_single_command() {
        let mut history = ReplHistory {
            history: Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE),
            curr_idx: 0,
        };

        history.push_command("let a = 10;".to_string());

        assert_eq!(history.history.len(), 1);
        assert_eq!(history.history[0], "let a = 10;");
    }

    #[test]
    fn push_multiple_commands() {
        let mut history = ReplHistory {
            history: Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE),
            curr_idx: 0,
        };

        history.push_command("first".to_string());
        history.push_command("second".to_string());
        history.push_command("third".to_string());

        assert_eq!(history.history.len(), 3);
        assert_eq!(history.history[0], "first");
        assert_eq!(history.history[1], "second");
        assert_eq!(history.history[2], "third");
    }

    #[test]
    fn navigate_backwards() {
        let mut history = ReplHistory {
            history: vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
            curr_idx: 3,
        };

        assert_eq!(history.back(), Some("third"));
        assert_eq!(history.back(), Some("second"));
        assert_eq!(history.back(), Some("first"));
        assert_eq!(history.back(), None);
    }

    #[test]
    fn navigate_forwards() {
        let mut history = ReplHistory {
            history: vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
            curr_idx: 0,
        };

        assert_eq!(history.next(), Some("second"));
        assert_eq!(history.next(), Some("third"));
        assert_eq!(history.next(), None);
    }

    #[test]
    fn empty_history_navigation() {
        let mut history = ReplHistory {
            history: Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE),
            curr_idx: 0,
        };

        assert_eq!(history.back(), None);
        assert_eq!(history.next(), None);
    }

    #[test]
    fn push_after_navigation() {
        let mut history = ReplHistory {
            history: vec!["first".to_string(), "second".to_string()],
            curr_idx: 1,
        };

        history.back();
        history.push_command("third".to_string());

        assert_eq!(history.history.len(), 3);
        assert_eq!(history.history[2], "third");
    }

    #[test]
    fn new_history_navigation_and_push() {
        let mut history = ReplHistory::new();

        let initial_len = history.history.len();

        history.push_command("test_command_1".to_string());
        history.push_command("test_command_2".to_string());

        assert_eq!(history.history.len(), initial_len + 2);

        assert_eq!(history.back(), Some("test_command_2"));
        assert_eq!(history.back(), Some("test_command_1"));

        history.push_command("test_command_3".to_string());

        assert_eq!(
            history.history.last().map(|s| s.as_str()),
            Some("test_command_3")
        );
    }
}
