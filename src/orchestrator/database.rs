use serde::{Deserialize, Serialize};

use super::Collection;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Database {
    name: String,
    collections: Vec<Collection>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Database {
            name: name,
            collections: Vec::new(),
        }
    }

    pub fn add_collection(&mut self, collection_name: &String) -> bool {
        self.collections
            .push(Collection::new(collection_name.to_string()));
        return true;
    }

    pub fn remove_collection(&mut self, collection_name: &String) -> bool {
        let removed = self
            .collections
            .iter()
            .position(|coll| coll.get_name() == *collection_name);
        if let Some(index) = removed {
            self.collections.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_collections(&self) -> Vec<Collection> {
        self.collections.clone()
    }

    pub fn get_collection(&self, name: &str) -> Collection {
        self.collections
            .iter()
            .find(|coll| coll.get_name().eq(name))
            .cloned()
            .unwrap_or_else(|| Collection::new(String::new()))
    }

    pub fn collection_exists(&self, database_name: &String) -> bool {
        self.collections
            .iter()
            .any(|db| db.get_name().eq(database_name))
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
