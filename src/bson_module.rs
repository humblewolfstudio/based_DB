use std::{
    fs::File,
    io::{Read, Write},
};

use bson::{bson, Array, Bson, Document};

pub fn string_to_bson(json: String) -> Bson {
    let bson_data = bson!(json);
    println!("BSON data: {:?}", bson_data.to_string());
    return bson_data;
}

pub async fn store_bson(_document: Bson) -> Result<String, String> {
    println!("Data: {:?}", _document.to_string());

    //Inicializamos un vector de Bson
    let bson_objects = Array::new();

    match serialize_bson_vector(bson_objects, _document).await {
        Ok(buf) => {
            match File::create("data.bson") {
                Ok(mut file) => match file.write_all(&buf) {
                    Ok(()) => {
                        match read_bson().await {
                            Ok(vec) => {
                                deserialize_bson(vec);
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

async fn deserialize_bson(serialized_data: Vec<u8>) -> Result<Vec<Bson>, String> {
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
