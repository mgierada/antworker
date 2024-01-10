#[derive(Debug, Default)]
pub struct EmailAccountBuilder {
    pub server: String,
    pub port: u16,
    pub email: String,
    pub password: String,
    pub uid_set: String,
}

impl EmailAccountBuilder {
    pub fn new(server: &str, port: u16, email: &str, password: &str) -> Self {
        EmailAccountBuilder {
            server: server.to_string(),
            port,
            email: email.to_string(),
            password: password.to_string(),
            uid_set: "1:*".to_string(), // Retrieve all emails by default
        }
    }

    pub fn uid_set(mut self, uid_set: &str) -> Self {
        self.uid_set = uid_set.to_string();
        self
    }

    pub fn build(self) -> EmailAccountBuilder {
        EmailAccountBuilder {
            server: self.server,
            port: self.port,
            email: self.email,
            password: self.password,
            uid_set: self.uid_set,
        }
    }
}
