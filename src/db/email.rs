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
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailMonthly {
    pub year_month: String,
    pub emails: Emails,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emails {
    pub mailbox: String,
    pub details: Vec<EmailDetails>,
}
