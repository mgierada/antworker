pub enum Tables {
    Emails,
}

impl std::fmt::Display for Tables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tables::Emails => write!(f, "emails"),
        }
    }
}
