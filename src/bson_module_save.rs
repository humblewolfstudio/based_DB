use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use bson::{bson, document, Array, Bson, Document};
use serde_json::Value;


//DEPRECATED

//TODO He estado trabajando con Bson cuando el objeto es el Document y bson la representacion de datos xd TODO MAL

pub fn string_to_bson(json: String) -> Bson {
    let bson_data = bson!(json);
    return bson_data;
}

pub async fn store_bson(_document: Bson) -> Result<String, String> {
    println!("Data: {:?}", _document.to_string());

    //Inicializamos un vector de Bson
    let bson_objects;

    //Leemos el bson en data.bson para poder poblar el array y asi insertar el nuevo documento con el resto
    match read_bson().await {
        Ok(vec) => {
            bson_objects = u8_to_bson(vec).await.unwrap();
        }
        Err(_e) => return Err("Error reading data".to_string()),
    };

    match serialize_bson_vector(bson_objects, _document).await {
        Ok(buf) => {
            match File::create("data.bson") {
                Ok(mut file) => match file.write_all(&buf) {
                    Ok(()) => {
                        match read_bson().await {
                            Ok(vec) => {
                                let objects = deserialize_bson(vec).await.unwrap();
                                println!("Data Stored: {:?}", objects);
                                return Ok("OK".to_string());
                            }
                            Err(_e) => return Err("Error reading data".to_string()),
                        };
                    }
                    Err(_e) => return Err("Error writing the bson to disk".to_string()),
                },
                Err(_e) => return Err("Error creating the bson file".to_string()),
            };
        }
        Err(_e) => return Err("Error serialising bson".to_string()),
    };
}

async fn serialize_bson_vector(
    mut bson_objects: Vec<Bson>,
    document: Bson,
) -> Result<Vec<u8>, String> {
    //Pusheamos el nuevo bson en el array
    bson_objects.push(document);
    //Inicializamos un array de documents
    let mut bson_document = Document::new();
    //Añadimos en el array de documentos nuestros bson
    bson_document.insert("array", bson_objects);

    // Serializamos los datos a Vec<u8>
    let buffer = bson::to_vec(&bson_document).expect("Failed to serialize BSON document");
    return Ok(buffer);
}

async fn u8_to_bson(serialized_data: Vec<u8>) -> Result<Vec<Bson>, String> {
    let bson_objects = Array::new();
    let bson_document: Document =
        bson::from_slice(&serialized_data).expect("Failed to deserialize BSON document");
    if let Some(Bson::Array(bson_array)) = bson_document.get("array") {
        // The BSON array is now in bson_array
        println!("{:?}", bson_array);
        return Ok(bson_objects);
    } else {
        println!("BSON document does not contain an array.");
        return Err("BSON document does not contain an array.".to_string());
    }
}

async fn deserialize_bson(serialized_data: Vec<u8>) -> Result<Vec<HashMap<String, Bson>>, String> {
    let bson_document: Document =
        bson::from_slice(&serialized_data).expect("Failed to deserialize BSON document");
    if let Some(Bson::Array(bson_array)) = bson_document.get("array") {
        // The BSON array is now in bson_array
        println!("Deserialized: {:?}", bson_array);
        //Convertimos de [String("noseuq")] a ["nose":"que"]
        let object_array = convert_deserialized_bson_array_to_objects(bson_array.to_owned());
        return Ok(object_array.to_owned());
    } else {
        println!("BSON document does not contain an array.");
        return Err("BSON document does not contain an array.".to_string());
    }
}

fn convert_deserialized_bson_array_to_objects(bson_array: Vec<Bson>) -> Vec<HashMap<String, Bson>> {
    //Definimos el vector donde se guardaran nuestros objetos en formato HashMap
    let mut object_vec: Vec<HashMap<String, Bson>> = Vec::new();
    //Para documento en el array los parseamos a un documento
    //Y despues iteramos por las claves de esos documentos para guardar en un hash (object_map) las key/values
    for bson_string in bson_array {
        let bson_document = string_to_json(bson_string.to_string());
        if let Bson::Document(document) = bson_document {
            let mut object_map: HashMap<String, Bson> = HashMap::new();

            for (key, value) in document {
                if let Bson::String(field_name) = Bson::String(key) {
                    println!("KEY: {}", &field_name);
                    println!("VALUE: {}", &value);
                    object_map.insert(field_name, value);
                }
            }

            object_vec.push(object_map);
        } else {
            //TODO salta aixo ja que reconeix el contingut com a string i no com a bson
            println!(
                "Coudlnt convert bson_document to document: {}",
                bson_document
            );
        }
    }

    return object_vec;
}

//TODO intentant solucionar el todo d'adalt amb aquesta funcio
fn string_to_json(json_str: String) -> Bson {
    return serde_json::from_str(&json_str).expect("Failed to parse JSON");
}

fn convert_deserialized_bson_to_objects(bson: Bson) -> Result<HashMap<String, Bson>, String> {
    if let Bson::Document(document) = bson {
        let mut object_map: HashMap<String, Bson> = HashMap::new();

        for (key, value) in document {
            if let Bson::String(field_name) = Bson::String(key) {
                object_map.insert(field_name, value);
            }
        }

        return Ok(object_map);
    }

    return Err("Couldnt convert bson to document".to_string());
}

pub async fn read_bson() -> Result<Vec<u8>, String> {
    match File::open("data.bson") {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_size) => return Ok(buffer),
                Err(_e) => return Err("Error reading file".to_string()),
            }
        }
        Err(_e) => return Err("Error opening file".to_string()),
    }
}
