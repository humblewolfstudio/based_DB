use tcp_server::get_data;

use crate::{
    bson_module::{
        read_collection_deserialized, search_in_vector_document, serialize_collection_to_string,
        string_to_document,
    },
    orchestrator_handler::Orchestrator,
};

pub async fn handle_find(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    let (database_name, collection_name, data) = get_data(message.to_vec());

    if orchestrator.database_exists(&database_name) {
        return Err("No database sent.".to_string());
    }

    let database = orchestrator.get_database(&database_name);

    if database.collection_exists(&collection_name) {
        return Err("No collection sent.".to_string());
    }

    let _collection = database.get_collection(&collection_name);

    if data.eq("") {
        return Err("No document sent.".to_string());
    }

    match string_to_document(data) {
        Ok(doc) => match read_collection_deserialized(&database_name, &collection_name).await {
            Ok(vec) => {
                let found = search_in_vector_document(vec, doc);
                match serialize_collection_to_string(found) {
                    Ok(vec) => return Ok(vec),
                    Err(_e) => {
                        return Err("Failed to stringify BSON documents to JSON.".to_string())
                    }
                }
            }
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    };
}
