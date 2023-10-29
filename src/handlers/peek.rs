use crate::orchestrator::Orchestrator;

pub fn handle_peek(orchestrator: &mut Orchestrator) -> String {
    return format!("[{}]", orchestrator.get_databases().join(", "));
}
