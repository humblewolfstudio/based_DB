use orchestrator::{Database, Orchestrator};

mod orchestrator;
pub fn get_data(data: Vec<&str>, orchestrator: &Orchestrator) -> (Database, String, String) {
    let database;
    let collection;
    let content;

    let len = data.len();

    if len <= 0 {
        database = Database::new(String::new());
        collection = String::new();
        content = String::new();
    } else if len < 2 {
        database = orchestrator.get_database(data[0]).to_owned();
        collection = String::new();
        content = String::new();
    } else if len >= 2 {
        database = orchestrator.get_database(data[0]).to_owned();
        collection = data[1].to_string();
        content = data[2..data.len()].join("");
    } else {
        database = Database::new(String::new());
        collection = String::new();
        content = String::new();
    }

    return (database, collection, content);
}
