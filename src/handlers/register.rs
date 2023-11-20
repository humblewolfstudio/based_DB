use bson::Bson;

use crate::{
    bson_module::string_to_document,
    orchestrator::{Orchestrator, User},
};

pub fn handle_register(
    message: &String,
    orchestrator: &mut Orchestrator,
    user: &User,
) -> Result<String, String> {
    if message.is_empty() {
        return Err("User JSON required.".to_string());
    }

    if !user.has_user_permission(super::Command::REGISTER) {
        return Err("This user cant REGISTER to the database".to_string());
    }

    let document;
    match string_to_document(message) {
        Ok(res) => document = res,
        Err(e) => return Err(e),
    }

    println!("REGISTER: {:?}", document);

    let username;
    let password;
    let permissions;
    let database;

    match document.get("username") {
        Some(Bson::String(_username)) => username = _username,
        Some(_) => return Err("Username has to be a String".to_string()),
        None => return Err("Username is missing".to_string()),
    }

    match document.get("password") {
        Some(Bson::String(_password)) => password = _password,
        Some(_) => return Err("Password has to be a String".to_string()),
        None => return Err("Password is missing".to_string()),
    }

    match document.get("permissions") {
        Some(Bson::Array(_permissions)) => {
            let found_permissions: Vec<String> = _permissions
                .iter()
                .filter_map(Bson::as_str)
                .map(ToString::to_string)
                .collect();

            permissions = found_permissions;
        }
        Some(_) => return Err("Permissions has to be an Array of String".to_string()),
        None => return Err("Permissions is missing".to_string()),
    }

    match document.get("database") {
        Some(Bson::String(_database)) => database = _database,
        Some(_) => return Err("Database has to be an string".to_string()),
        None => return Err("Database is missing".to_string()),
    }

    match orchestrator.create_user(username, password, database, permissions) {
        Ok(_ok) => return Ok("OK".to_string()),
        Err(e) => return Err(e),
    }
}
