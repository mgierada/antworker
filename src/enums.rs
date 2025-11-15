pub enum OpenCommand {
    Income,
    Outcome,
}

#[derive(Debug)]
pub enum DbAction {
    GetEmail,
    RemoveEmail,
    GetMailbox,
    RemoveMailbox,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_command_variants() {
        // Test that we can create both variants
        let _income = OpenCommand::Income;
        let _outcome = OpenCommand::Outcome;
    }

    #[test]
    fn test_db_action_debug() {
        // Test Debug trait implementation
        let get_email = DbAction::GetEmail;
        let debug_str = format!("{:?}", get_email);
        assert_eq!(debug_str, "GetEmail");

        let remove_email = DbAction::RemoveEmail;
        let debug_str = format!("{:?}", remove_email);
        assert_eq!(debug_str, "RemoveEmail");

        let get_mailbox = DbAction::GetMailbox;
        let debug_str = format!("{:?}", get_mailbox);
        assert_eq!(debug_str, "GetMailbox");

        let remove_mailbox = DbAction::RemoveMailbox;
        let debug_str = format!("{:?}", remove_mailbox);
        assert_eq!(debug_str, "RemoveMailbox");
    }
}
