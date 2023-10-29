use crate::orchestrator::Orchestrator;

pub async fn handle_update(
    database: String,
    _data: String,
    orchestrator: &mut Orchestrator,
) -> Result<String, String> {
    if !&orchestrator.database_exists(&database) {
        return Err("Database not recognized".to_string());
    }

    return Err("Unimplemented".to_string());
}
