use crate::{
    crud::get_email::get_emails_ids_for_current_year_month,
    db::{connect::connect, email::EmailMonthly, enums::Tables},
};

pub async fn remove_emails() -> surrealdb::Result<()> {
    let db = connect().await?;
    let ids_to_remove = get_emails_ids_for_current_year_month().await?;
    for id in &ids_to_remove {
        let _deleted_email: Option<EmailMonthly> =
            db.delete((Tables::Emails.to_string(), id.clone())).await?;
    }
    Ok(())
}
