use crate::orchestrator::Orchestrator;

pub fn handle_create(database: String, orchestrator: &mut Orchestrator) -> Result<String, String> {
    return orchestrator.create_database(database);
}
