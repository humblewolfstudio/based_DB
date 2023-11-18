use crate::orchestrator::Orchestrator;

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
        print!("message: {}", message[0]);
        return format!(
            "[{}]",
            orchestrator
                .get_database(message[0])
                .get_collections()
                .iter()
                .map(|coll| coll.get_name())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}
