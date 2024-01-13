use crate::factories::credentials::EmailAccountBuilder;

#[test]
fn test_new() {
    let builder = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password");

    assert_eq!(builder.server, "smtp.example.com");
    assert_eq!(builder.port, 587);
    assert_eq!(builder.email, "user@example.com");
    assert_eq!(builder.password, "password");
    assert_eq!(builder.uid_set, "1:*");
}

#[test]
fn test_uid_set() {
    let builder = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("100:200");

    assert_eq!(builder.uid_set, "100:200");
}

#[test]
fn test_build() {
    let original =
        EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
            .uid_set("100:200");

    let built = original.clone().build();

    assert_eq!(built, original);
}
