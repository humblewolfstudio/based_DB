use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    username: String,
    hashed_pw: String,
    permissions: Vec<String>,
    database: String,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        permissions: Vec<String>,
        database: String,
    ) -> Self {
        User {
            username,
            hashed_pw: password,
            permissions,
            database: database,
        }
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_password(&self) -> String {
        self.hashed_pw.to_string()
    }
}
