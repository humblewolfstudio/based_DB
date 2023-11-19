use crate::{
    bson_module::{
        delete_collection, delete_in_vector_document, read_collection_deserialized,
        serialize_collection, store_collection, string_to_document,
    },
    orchestrator::Orchestrator,
};

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
            match string_to_document(data) {
                Ok(doc) => {
                    match read_collection_deserialized(&database_name, &collection_name).await {
                        Ok(mut vec) => match delete_in_vector_document(&mut vec, doc) {
                            Ok(_e) => match serialize_collection(vec) {
                                Ok(ser_vec) => match store_collection(
                                    ser_vec,
                                    &database_name,
                                    &collection_name,
                                )
                                .await
                                {
                                    Ok(res) => return Ok(res),
                                    Err(e) => return Err(e),
                                },
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(e),
                        },
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
    } else {
        return Err("Database doesnt exist.".to_string());
    }
}
