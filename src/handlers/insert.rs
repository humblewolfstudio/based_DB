use crate::{bson_module, bson_module::store_document, orchestrator::Orchestrator};

//Convertim el string entrant en un document, llegim la coleccio guardada e insertem el document en la coleccio
pub async fn handle_insert(
    database: String,
    collection: String,
    data: String,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
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
