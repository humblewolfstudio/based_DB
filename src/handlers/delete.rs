use crate::{bson_module::delete_collection, orchestrator::Orchestrator};

use super::get_data;

pub async fn handle_delete(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    let (database_name, collection_name, data) = get_data(message.to_vec());

    if database_name.is_empty() {
        return Err("No database sent".to_string());
    }

    if collection_name.is_empty() {
        return Err("No collection sent".to_string());
    }

    if let Some(database) = orchestrator.get_database(&database_name) {
        if data.is_empty() {
            //If no data, we remove the collection
            if !database.collection_exists(&collection_name) {
                return Err("Collection doesnt exists".to_string());
            } else {
                database.remove_collection(&collection_name);

                return delete_collection(&database_name, &collection_name).await;
            }
        } else {
            //If theres data, we remove the documents inside the collection
            return Ok("Not implemented...".to_string()); //TODO implement removing of documents inside collection
        }
    } else {
        return Err("Database doesnt exist.".to_string());
    }
}
