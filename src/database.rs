use std::{
    fs::{self},
    ops::Add,
};

pub fn create_database(database_name: &String) -> Result<String, String> {
    let dir_name = database_name.to_string();
    match fs::create_dir("data/".to_string().add(&dir_name)) {
        Ok(_file) => {
            return Ok("OK".to_string());
        }
        Err(_e) => {
            println!("ERROR: {:?}", _e);
            return Err("Error creating file".to_string());
        }
    }
}
