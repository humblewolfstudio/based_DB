use crate::Orchestrator;

pub fn handle_peek(message: &Vec<&str>, orchestrator: &mut Orchestrator) -> String {
    if message.len() < 1 {
        return format!(
            "[{}]",
            orchestrator
                .get_databases()
                .iter()
                .map(|db| db.get_name().clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
    } else {
        if let Some(database) = orchestrator.get_database(&message[0]) {
            return format!(
                "[{}]",
                database
                    .get_collections()
                    .iter()
                    .map(|coll| coll.get_name())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        } else {
            return "Database doesnt exist.".to_string();
        }
    }
}
