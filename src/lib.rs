pub mod orchestrator_handler;

pub use orchestrator_handler::{Collection, Database, Orchestrator};

mod bson_module;

pub fn get_data(data: Vec<&str>) -> (String, String, String) {
    let database;
    let collection;
    let content;
    
    let len = data.len();

    if len <= 0 {
        database = String::new();
        collection = String::new();
        content = String::new();
    } else if len == 1 {
        database = data[0].to_string();
        collection = String::new();
        content = String::new();
    } else if len == 2 {
        database = data[0].to_string();
        collection = data[1].to_string();
        content = String::new();
    } else if len >= 2 {
        database = data[0].to_string();
        collection = data[1].to_string();
        content = data[2..data.len()].join("");
    } else {
        database = String::new();
        collection = String::new();
        content = String::new();
    }

    return (database, collection, content);
}
