use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env::var;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::email_parser::parser::EmailDetails;

#[derive(Debug, Serialize)]
struct HistoryPerYearMonth<'a> {
    year_month: &'a str,
    send_history: Vec<SendHistory<'a>>,
}

#[derive(Debug, Serialize)]
struct SendHistory<'a> {
    mailbox: &'a str,
    email_details: EmailDetails,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

lazy_static! {
    static ref DB_HOST: String = var("DB_HOST").expect("DB_HOST must be set.");
}

lazy_static! {
    static ref DB_PORT: String = var("DB_PORT").expect("DB_PORT must be set.");
}

lazy_static! {
    static ref DB_USERNAME: String = var("DB_USERNAME").expect("DB_USERNAME must be set.");
}

lazy_static! {
    static ref DB_PASSWORD: String = var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
}

lazy_static! {
    static ref DB_NAME: String = var("DB_NAME").expect("DB_NAME must be set.");
}

lazy_static! {
    static ref DB_NAMESPACE: String = var("DB_NAMESPACE").expect("DB_NAMESPACE must be set.");
}

pub async fn connect() -> Result<Surreal<surrealdb::engine::remote::ws::Client>, surrealdb::Error> {
    let connect_url = format!("{}:{}", *DB_HOST, *DB_PORT);
    let db = Surreal::new::<Ws>(connect_url).await?;
    db.signin(Root {
        username: &DB_USERNAME,
        password: &DB_PASSWORD,
    })
    .await?;
    db.use_ns(DB_NAMESPACE.to_string())
        .use_db(DB_NAME.to_string())
        .await?;
    Ok(db)
}

pub async fn create() -> surrealdb::Result<()> {
    let db = connect().await?;
    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("send_history")
        .content(HistoryPerYearMonth {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(())
}
