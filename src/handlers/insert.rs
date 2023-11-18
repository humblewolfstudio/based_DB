use tcp_server::get_data;

use crate::{bson_module, bson_module::store_document, orchestrator::Orchestrator};

//Convertim el string entrant en un document, llegim la coleccio guardada e insertem el document en la coleccio
pub async fn handle_insert(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    let (database, collection, data) = get_data(message.to_vec(), orchestrator);
    //We check if db, collection and data are present. If not, we return an error
    if database.eq("") {
        return Err("No database sent.".to_string());
    }

    if collection.eq("") {
        return Err("No collection sent.".to_string());
    }

    if data.eq("") {
        return Err("No document sent.".to_string());
    }

    let document;

    if !&orchestrator.database_exists(&database) {
        return Err("Database not recognized".to_string());
    }

    match bson_module::string_to_document(data) {
        Ok(res) => document = res,
        Err(e) => return Err(e),
    }

    match bson_module::read_collection_deserialized(&database, &collection).await {
        Ok(vec) => match store_document(document, vec, &database, &collection).await {
            Ok(res) => return Ok(res),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}
