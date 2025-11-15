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

#[test]
fn test_new_with_different_ports() {
    let builder_587 = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "pass");
    assert_eq!(builder_587.port, 587);

    let builder_465 = EmailAccountBuilder::new("smtp.example.com", 465, "user@example.com", "pass");
    assert_eq!(builder_465.port, 465);

    let builder_993 = EmailAccountBuilder::new("imap.example.com", 993, "user@example.com", "pass");
    assert_eq!(builder_993.port, 993);
}

#[test]
fn test_uid_set_with_various_patterns() {
    let builder1 = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("1:10");
    assert_eq!(builder1.uid_set, "1:10");

    let builder2 = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("50:*");
    assert_eq!(builder2.uid_set, "50:*");

    let builder3 = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("100");
    assert_eq!(builder3.uid_set, "100");
}

#[test]
fn test_builder_chaining() {
    let builder = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("100:200")
        .build();

    assert_eq!(builder.server, "smtp.example.com");
    assert_eq!(builder.port, 587);
    assert_eq!(builder.email, "user@example.com");
    assert_eq!(builder.password, "password");
    assert_eq!(builder.uid_set, "100:200");
}

#[test]
fn test_default_implementation() {
    let default_builder = EmailAccountBuilder::default();
    assert_eq!(default_builder.server, "");
    assert_eq!(default_builder.port, 0);
    assert_eq!(default_builder.email, "");
    assert_eq!(default_builder.password, "");
    assert_eq!(default_builder.uid_set, "");
}

#[test]
fn test_clone_equality() {
    let builder1 = EmailAccountBuilder::new("smtp.example.com", 587, "user@example.com", "password")
        .uid_set("100:200");
    let builder2 = builder1.clone();

    assert_eq!(builder1, builder2);
    assert_eq!(builder1.server, builder2.server);
    assert_eq!(builder1.port, builder2.port);
    assert_eq!(builder1.email, builder2.email);
    assert_eq!(builder1.password, builder2.password);
    assert_eq!(builder1.uid_set, builder2.uid_set);
}
