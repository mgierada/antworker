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
