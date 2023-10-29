use tcp_server::get_data;

use crate::{
    bson_module::{
        read_collection_deserialized, search_in_vector_document, serialize_collection_to_string,
        string_to_document,
    },
    orchestrator::Orchestrator,
};

pub async fn handle_find(
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

    match string_to_document(data) {
        Ok(doc) => match read_collection_deserialized(&database, &collection).await {
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
