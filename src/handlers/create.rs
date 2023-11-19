use crate::orchestrator::{orchestrator::Orchestrator, User};

use super::get_data;

pub fn handle_create(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
    user: &User,
) -> Result<String, String> {
    let (database, _collection, _data) = get_data(message.to_vec());
    if database.eq("") {
        return Err("Database Name cant be an empty string".to_string());
    }

    if !user.is_database_in_user(&database) {
        return Err("This user cant interact with this database".to_string());
    }

    if !user.has_user_permission(super::Command::CREATE) {
        return Err("This user cant CREATE database".to_string());
    }

    return orchestrator.create_database(database);
}
