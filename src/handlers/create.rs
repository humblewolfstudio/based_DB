use crate::orchestrator::orchestrator::Orchestrator;

use super::aux_fn::get_data;

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
