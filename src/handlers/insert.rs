use super::get_data;

use crate::{
    bson_module,
    bson_module::store_document,
    orchestrator::{orchestrator::Orchestrator, User},
};

//Convertim el string entrant en un document, llegim la coleccio guardada e insertem el document en la coleccio
pub async fn handle_insert(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
    user: &User,
) -> Result<String, String> {
    let (database_name, collection_name, data) = get_data(message.to_vec());

    if database_name.is_empty() {
        return Err("Database not sent.".to_string());
    }

    if !user.is_database_in_user(&database_name) {
        return Err("This user cant interact with this database".to_string());
    }

    if !user.has_user_permission(super::Command::INSERT) {
        return Err("This user cant INSERT to the database".to_string());
    }

    if let Some(database) = orchestrator.get_database(&database_name) {
        if collection_name.is_empty() {
            return Err("You have to send a collection name".to_string());
        }

        if !database.collection_exists(&collection_name) {
            //Si no existe la coleccion, que se añada en el coso ese
            database.add_collection(&collection_name);
            let _ = orchestrator.save_orchestrator();
        }

        if data.is_empty() {
            return Err("No document sent.".to_string());
        }

        let document;

        match bson_module::string_to_document(&data) {
            Ok(res) => document = res,
            Err(e) => return Err(e),
        }

        match bson_module::read_collection_deserialized(&database_name, &collection_name).await {
            Ok(vec) => {
                match store_document(document, vec, &database_name, &collection_name).await {
                    Ok(res) => return Ok(res),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    } else {
        return Err("Database doesnt exist.".to_string());
    }
}
