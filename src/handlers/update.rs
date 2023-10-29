use tcp_server::get_data;

use crate::orchestrator::Orchestrator;

pub async fn handle_update(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    let (database, collection, data) = get_data(message.to_vec());

    if database.eq("") {
        return Err("No database sent.".to_string());
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
