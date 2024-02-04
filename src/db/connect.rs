use lazy_static::lazy_static;
use std::env::var;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

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
    let db = Surreal::new::<Ws>(connect_url).await.unwrap_or_else(|e| {
        panic!(
            "Could not connect to the database.\nError:\n{:?}",
            e.to_string()
        )
    });
    db.signin(Root {
        username: &DB_USERNAME,
        password: &DB_PASSWORD,
    })
    .await.unwrap_or_else(
        |e| panic!("Could not sign in to the database. Error: {:?}", e),
    );
    db.use_ns(DB_NAMESPACE.to_string())
        .use_db(DB_NAME.to_string())
        .await?;
    Ok(db)
}

