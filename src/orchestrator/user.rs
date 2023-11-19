use serde::{Deserialize, Serialize};

use crate::handlers::Command;

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

    pub fn is_database_in_user(&self, database_name: &String) -> bool {
        return self.database.eq("*") || self.database.eq(database_name);
    }

    pub fn has_user_permission(&self, handler: Command) -> bool {
        return self.permissions.contains(&"*".to_string())
            || self.permissions.contains(&handler.to_string());
    }
}
