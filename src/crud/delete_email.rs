use crate::crud::get_email::EmailDatabase;
use crate::crud::get_email::MyDatabaseConnection;
use crate::db::connect::connect;
use crate::db::email::EmailMonthly;
use crate::db::enums::Tables;

pub async fn remove_emails() -> surrealdb::Result<()> {
    let db = connect().await?;
    let db_conn = MyDatabaseConnection;
    let ids_to_remove = db_conn.get_emails_ids_for_current_year_month().await?;
    for id in &ids_to_remove {
        let _deleted_email: Option<EmailMonthly> =
            db.delete((Tables::Emails.to_string(), id.clone())).await?;
    }
    Ok(())
}
