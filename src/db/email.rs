use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::email_parser::parser::EmailDetails;


#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMonthly {
    pub id: Thing,
    pub year_month: String,
    pub emails: Emails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailMonthly {
    pub year_month: String,
    pub emails: Emails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails {
    pub mailbox: String,
    pub details: Vec<EmailDetails>,
}
