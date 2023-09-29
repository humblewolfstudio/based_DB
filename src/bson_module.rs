use std::{
    fs::File,
    io::{Read, Write},
};

use bson::{from_reader, from_slice, Document};
use serde_json::Value;

pub async fn store_document(document: Document, mut vec: Vec<Document>) -> Result<String, String> {
    vec.push(document);

    match serialize_collection(vec) {
        Ok(serialized_data) => match store_collection(serialized_data).await {
            Ok(res) => return Ok(res),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

pub async fn store_collection(vec: Vec<u8>) -> Result<String, String> {
    match File::create("data.bson") {
        Ok(mut file) => {
            file.write_all(&vec).expect("Error writing to file");
            return Ok("OK".to_string());
        }
        Err(_e) => return Err("Error creating file".to_string()),
    }
}

pub async fn read_collection() -> Result<Vec<u8>, String> {
    match File::open("data.bson") {
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

pub async fn read_collection_deserialized() -> Result<Vec<Document>, String> {
    match read_collection().await {
        Ok(vec) => match deserialize_collection(vec) {
            Ok(data) => return Ok(data),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

pub fn string_to_document(string: String) -> Result<Document, String> {
    //let json_value = serde_json::from_str(&string);
    match serde_json::from_str::<Value>(&string) {
        Ok(json_value) => {
            let bson_doc = bson::to_document(&json_value).expect("Failed");
            return Ok(bson_doc);
        }
        Err(e) => return Err("Error parsing string".to_string()),
    }
}

fn serialize_collection(documents: Vec<Document>) -> Result<Vec<u8>, String> {
    let mut serialized_vector: Vec<u8> = documents
        .iter()
        .map(|doc| bson::to_vec(doc).expect("Failed to serialize BSON in collection"))
        .collect::<Vec<Vec<u8>>>()
        .concat();
    return Ok(serialized_vector);
}

fn serialize_document(document: Document) -> Result<Vec<u8>, String> {
    let mut serialized_data: Vec<u8> = Vec::new();
    document
        .to_writer(&mut serialized_data)
        .expect("Failed to serialize BSON");

    return Ok(serialized_data);
}

fn deserialize_document(vec: Vec<u8>) -> Result<Document, String> {
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
