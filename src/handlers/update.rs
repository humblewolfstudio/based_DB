use crate::orchestrator::{Orchestrator, User};

use super::get_data;

pub async fn handle_update(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
    user: &User,
) -> Result<String, String> {
    let (database, collection, data) = get_data(message.to_vec());

    if database.eq("") {
        return Err("No database sent.".to_string());
    }

    if !user.is_database_in_user(&database) {
        return Err("This user cant interact with this database".to_string());
    }

    if !user.has_user_permission(super::Command::UPDATE) {
        return Err("This user cant UPDATE this database".to_string());
    }

    if collection.eq("") {
        return Err("No collection sent.".to_string());
    }

    if data.eq("") {
        return Err("No document sent.".to_string());
    }

    if !&orchestrator.database_exists(&database) {
        return Err("Database not recognized".to_string());
    }

    return Err("Unimplemented".to_string());
}
