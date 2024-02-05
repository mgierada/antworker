use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMailboxMonthly {
    pub year_month: String,
    pub updated_at: String,
    pub latest_uid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxMonthly {
    pub id: Thing,
    pub year_month: String,
    pub updated_at: String,
    pub latest_uid: u32,
}
