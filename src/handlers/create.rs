use tcp_server::get_data;

use crate::orchestrator::Orchestrator;

pub fn handle_create(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    let (database, _collection, _data) = get_data(message.to_vec());
    if database.eq("") {
        return Err("Database Name cant be an empty string".to_string());
    }

    return orchestrator.create_database(database);
}
