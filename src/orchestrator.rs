use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

use crate::bson_module;

//Le hacemos el Clone y el Copy para que pueda hacerse borrow
#[derive(Serialize, Deserialize, Clone)]
pub struct Orchestrator {
    databases: Vec<String>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            databases: vec!["test".to_string()],
        }
    }

    pub fn get_databases(&self) -> &Vec<String> {
        &self.databases
    }

    pub fn database_exists(&self, database_name: String) -> bool {
        return self.databases.contains(&database_name);
    }

    pub fn create_database(&mut self, database_name: String) -> Result<String, String> {
        self.databases.push(database_name);
        return self.save_orchestrator();
    }

    pub fn save_orchestrator(&self) -> Result<String, String> {
        match store_orchestrator(self.to_owned()) {
            Ok(res) => return Ok(res),
            Err(e) => return Err(e),
        }
    }
}

pub fn load_orchestrator() -> Result<Orchestrator, String> {
    let vec;
    let document;

    match read_file() {
        Ok(doc) => vec = doc,
        Err(_e) => return Ok(Orchestrator::new()),
    }

    if vec.is_empty() {
        return Ok(Orchestrator::new());
    }

    match bson_module::deserialize_document(vec) {
        Ok(data) => document = data,
        Err(_e) => return Ok(Orchestrator::new()),
    }

    match bson::from_document::<Orchestrator>(document) {
        Ok(data) => return Ok(data),
        Err(_e) => return Ok(Orchestrator::new()),
    }
}

pub fn store_orchestrator(orchestrator: Orchestrator) -> Result<String, String> {
    let document;
    let vec;
    //Convertimos el orchestrator a documento
    match bson::to_document(&orchestrator) {
        Ok(doc) => document = doc,
        Err(e) => return Err(e.to_string()),
    }

    match bson_module::serialize_document(document) {
        Ok(data) => vec = data,
        Err(e) => return Err(e),
    }

    match store_file(vec) {
        Ok(res) => return Ok(res),
        Err(e) => return Err(e),
    }
}

fn store_file(vec: Vec<u8>) -> Result<String, String> {
    match File::create("data/orchestrator.bson") {
        Ok(mut file) => {
            file.write_all(&vec).expect("Error writing to file");
            return Ok("OK".to_string());
        }
        Err(_e) => return Err("Error creating file".to_string()),
    }
}

fn read_file() -> Result<Vec<u8>, String> {
    match File::open("data/orchestrator.bson") {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_usize) => return Ok(buffer),
                Err(_e) => return Err("Error reading file".to_string()),
            }
        }
        Err(_e) => return Err("Error opening file".to_string()),
    }
}
