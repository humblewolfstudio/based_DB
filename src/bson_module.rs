use std::{
    collections::HashMap,
    fs::{remove_file, File},
    io::{Read, Write},
    ops::Add,
};

use bson::{from_reader, from_slice, Bson, Document};
use serde_json::{Error, Value};

pub async fn store_document(
    document: Document,
    mut vec: Vec<Document>,
    database: &String,
    collection: &String,
) -> Result<String, String> {
    //Si el documento esta vacio devolvemos un error
    if document.is_empty() {
        return Err("The document you're trying to insert is empty".to_string());
    }
    //Sino lo añadimos en el vector y lo serializamos
    vec.push(document);

    match serialize_collection(vec) {
        Ok(serialized_data) => {
            match store_collection(serialized_data, database, collection).await {
                Ok(res) => return Ok(res),
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}

pub async fn store_collection(
    vec: Vec<u8>,
    database: &String,
    collection: &String,
) -> Result<String, String> {
    //esto genera el archivo a leer (/data/{database}/{collection}.bson)
    let file_name = database.to_string() + "/" + collection + ".bson";
    match File::create("data/".to_string().add(&file_name)) {
        Ok(mut file) => {
            file.write_all(&vec).expect("Error writing to file");
            return Ok("OK".to_string());
        }
        Err(_e) => return Err("Error creating file".to_string()),
    }
}

pub async fn read_collection(database: &String, collection: &String) -> Result<Vec<u8>, String> {
    let file_name = database.to_string() + "/" + collection + ".bson";
    match File::open("data/".to_string().add(&file_name)) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_usize) => return Ok(buffer),
                Err(_e) => return Err("Error reading file".to_string()),
            }
        }
        Err(_e) => return Ok(Vec::new()),
    }
}

pub async fn delete_collection(database: &String, collection: &String) -> Result<String, String> {
    let file_name = "data/".to_string() + database + "/" + collection + ".bson";
    match remove_file(file_name) {
        Ok(_) => return Ok("OK".to_string()),
        Err(e) => {
            eprintln!("Error deleting file: {}", e);
            return Err("Error deleting collection".to_string());
        } //TODO we should log all the errors
    }
}

pub async fn read_collection_deserialized(
    database: &String,
    collection: &String,
) -> Result<Vec<Document>, String> {
    match read_collection(database, collection).await {
        Ok(vec) => match deserialize_collection(vec) {
            Ok(data) => return Ok(data),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

pub fn string_to_document(string: &String) -> Result<Document, String> {
    //let json_value = serde_json::from_str(&string);
    match serde_json::from_str::<Value>(&string) {
        Ok(json_value) => {
            let bson_doc = bson::to_document(&json_value).expect("Failed");
            return Ok(bson_doc);
        }
        Err(_e) => return Err("Error parsing string".to_string()),
    }
}

pub fn serialize_collection(documents: Vec<Document>) -> Result<Vec<u8>, String> {
    let serialized_vector: Vec<u8> = documents
        .iter()
        .map(|doc| bson::to_vec(doc).expect("Failed to serialize BSON in collection"))
        .collect::<Vec<Vec<u8>>>()
        .concat();
    return Ok(serialized_vector);
}

pub fn serialize_collection_to_string(documents: Vec<Document>) -> Result<String, Error> {
    let json_objects: Vec<Value> = documents
        .iter()
        .map(|doc| bson_to_json_object(doc))
        .collect();

    let json_array = Value::Array(json_objects);

    return serde_json::to_string(&json_array);
}

fn bson_to_json_object(doc: &Document) -> Value {
    //Inicializamos un nuevo json
    let mut json_obj = serde_json::Map::new();

    for (key, value) in doc.iter() {
        //Por cada key, lo insertamos en el json con el valor pasado a su de bson a el correspondiente
        if let Ok(json_value) = bson_value_to_json_value(value) {
            json_obj.insert(key.clone(), json_value);
        }
    }

    Value::Object(json_obj)
}

fn bson_value_to_json_value(bson_value: &Bson) -> Result<Value, serde_json::Error> {
    // Convertimos un valor de bson a uno "normal"xd
    return serde_json::to_value(bson_value);
}

pub fn serialize_document(document: Document) -> Result<Vec<u8>, String> {
    let mut serialized_data: Vec<u8> = Vec::new();
    document
        .to_writer(&mut serialized_data)
        .expect("Failed to serialize BSON");

    return Ok(serialized_data);
}

pub fn deserialize_document(vec: Vec<u8>) -> Result<Document, String> {
    let document = from_reader(&vec[..]).expect("Failed to deserialize BSON");
    return Ok(document);
}

fn deserialize_collection(data: Vec<u8>) -> Result<Vec<Document>, String> {
    let mut deserialized_documents: Vec<Document> = Vec::new();

    let mut offset = 0;

    while offset < data.len() {
        //Le Bytes
        //Miramos cuanto ocupa el BSON
        let size = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        //Cogemos el chunk que hemos calculado que ocupa
        let chunk = &data[offset..offset + size];
        //Lo convertimos a documento
        let document = from_slice(chunk).expect("Failed to deserialize BSON in collection");
        //Lo añadimos al vector
        deserialized_documents.push(document);
        //Añadimos la size al offset
        offset += size;
    }

    return Ok(deserialized_documents);
}

fn get_document_keys(doc: &Document) -> HashMap<String, Bson> {
    let mut hashmap: HashMap<String, Bson> = HashMap::new();
    for (key, value) in doc.iter() {
        hashmap.insert(key.to_owned(), value.to_owned());
    }

    return hashmap;
}

pub fn search_in_vector_document(vector: Vec<Document>, doc: Document) -> Vec<Document> {
    let document_hashmap = get_document_keys(&doc);
    let mut found_vector: Vec<Document> = Vec::new();

    if document_hashmap.is_empty() {
        //si esta vacia el documento entrante, devolvemos todos los documentos
        return vector;
    }

    for document in vector {
        for (key, value) in document_hashmap.iter() {
            //de moment si te una sola key ja ens val xd
            if document.contains_key(key) {
                if document.get(key) == Some(value) {
                    found_vector.push(document.clone());
                }
            }
        }
    }

    return found_vector;
}

pub fn delete_in_vector_document(vector: &mut Vec<Document>, doc: Document) -> Result<String, String> {
    let document_hashmap = get_document_keys(&doc);

    if document_hashmap.is_empty() {
        return Err("No conditions in delete sent.".to_string());
    }

    vector.retain(|document| {
        for (key, value) in document_hashmap.iter() {
            if document.contains_key(key) && document.get(key) == Some(value) {
                return false;
            }
        }
        return true;
    });

    return Ok("OK".to_string());
}
