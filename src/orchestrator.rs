use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use crate::bson_module;
use crate::database;
use crypto_hash::{hex_digest, Algorithm};

//Le hacemos el Clone y el Copy para que pueda hacerse borrow
#[derive(Serialize, Deserialize, Clone)]
pub struct Orchestrator {
    databases: Vec<String>, //Vec of databases
    users: Vec<User>,       //Vec of users
    secure: bool,           //boolean to use or not use users to authenticate
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    hashed_pw: String,
    permissions: Vec<String>,
}

impl User {
    fn new(username: String, password: String, permissions: Vec<String>) -> Self {
        User {
            username,
            hashed_pw: password,
            permissions,
        }
    }
}

impl Orchestrator {
    pub fn new(secure: bool) -> Self {
        if !secure {
            println!("ALERT!\n-------------------------------------------------
            \nThe Orchestrator is in insecure mode. \nAnybody can connect to it. \nCreate a new user and change it to secure \n-------------------------------------------------");
        } else {
            println!("Orchestrator in secure mode.")
        }
        Orchestrator {
            databases: Vec::new(),
            users: vec![User::new(
                //Supervisor is supervisor:1234
                "supervisor".to_string(),
                "03ac674216f3e15c761ee1a5e255f067953623c8b388b4459e13f978d7c846f4".to_string(),
                Vec::new(),
            )],
            secure,
        }
    }

    pub fn authenticate_user(
        &self,
        username: &String,
        password: &String,
    ) -> Result<Vec<String>, String> {
        if &self.secure == &false {
            let empty_vec: Vec<String> = Vec::new();
            return Ok(empty_vec);
        }
        let hashed_pw;
        match hash_string(password, Algorithm::SHA256) {
            Ok(hashed_res) => hashed_pw = hashed_res,
            Err(_e) => return Err("Error hashing the password".to_string()),
        }

        for user in &self.users {
            if username == &user.username {
                if hashed_pw != user.hashed_pw {
                    return Err("Couldn't authenticate: Incorrect Password".to_string());
                }
                return Ok(user.permissions.clone());
            }
        }

        return Err("Couldn't authenticate: User not found".to_string());
    }

    pub fn get_databases(&self) -> &Vec<String> {
        &self.databases
    }

    pub fn database_exists(&self, database_name: &String) -> bool {
        return self.databases.contains(&database_name);
    }

    pub fn create_database(&mut self, database_name: String) -> Result<String, String> {
        self.databases.push(database_name.to_owned());
        match database::create_database(&database_name) {
            Ok(_ok) => return self.save_orchestrator(),
            Err(err) => return Err(err),
        }
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
        Err(_e) => return Ok(Orchestrator::new(true)), //TODO is secure because of this
    }

    if vec.is_empty() {
        return Ok(Orchestrator::new(false));
    }

    match bson_module::deserialize_document(vec) {
        Ok(data) => document = data,
        Err(_e) => return Ok(Orchestrator::new(false)),
    }

    match bson::from_document::<Orchestrator>(document) {
        Ok(data) => return Ok(data),
        Err(_e) => return Ok(Orchestrator::new(false)),
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

fn hash_string(input: &str, algorithm: Algorithm) -> Result<String, Box<dyn Error>> {
    let hash = hex_digest(algorithm, input.as_bytes());
    Ok(hash)
}
