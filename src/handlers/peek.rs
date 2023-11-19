use crate::{orchestrator::User, Orchestrator};

pub fn handle_peek(
    message: &Vec<&str>,
    orchestrator: &mut Orchestrator,
    user: &User,
) -> Result<String, String> {
    if !user.has_user_permission(super::Command::PEEK) {
        return Err("This user cant PEEK to this database".to_string());
    }

    if message.len() < 1 {
        return Ok(format!(
            "[{}]",
            orchestrator
                .get_databases()
                .iter()
                .map(|db| db.get_name().clone())
                .collect::<Vec<String>>()
                .join(", ")
        ));
    } else {
        if let Some(database) = orchestrator.get_database(&message[0]) {
            if !user.is_database_in_user(&message[0].to_string()) {
                return Err("This user cant interact with this database".to_string());
            }
            return Ok(format!(
                "[{}]",
                database
                    .get_collections()
                    .iter()
                    .map(|coll| coll.get_name())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        } else {
            return Err("Database doesnt exist.".to_string());
        }
    }
}
