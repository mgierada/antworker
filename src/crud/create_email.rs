use crate::{
    datemath::date::get_current_year_month_str,
    db::{
        connect::connect,
        email::{CreateEmailMonthly, EmailMonthly, Emails, Record},
        enums::Tables,
    },
};

pub async fn store_emails(emails: Emails) -> surrealdb::Result<()> {
    let db = connect().await?;
    let _: Vec<Record> = db
        .create(Tables::Emails.to_string())
        .content(CreateEmailMonthly{
            year_month: get_current_year_month_str(),
            emails,
        })
        .await?;
    Ok(())
}
