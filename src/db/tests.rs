use crate::db::connect::connect;

#[tokio::test]
async fn test_connect() {
    // Set up environment variables for testing
    std::env::set_var("DB_HOST", "127.0.0.1");
    std::env::set_var("DB_PORT", "8000");
    std::env::set_var("DB_USERNAME", "root");
    std::env::set_var("DB_PASSWORD", "root");
    std::env::set_var("DB_NAME", "antowrker");
    std::env::set_var("DB_NAMESPACE", "emails");

    let result = connect().await;
    assert!(result.is_ok(), "Connection failed: {:?}", result.err());
}
