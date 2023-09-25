use bson::{bson, Bson};

pub fn string_to_bson(json: String) -> Bson {
    let bson_data = bson!(json);
    println!("BSON data: {:?}", bson_data.to_string());
    return bson_data;
}

pub async fn store_bson(_document: Bson) -> Result<String, String> {
    println!("Data: {:?}", _document.to_string());
    return Ok("OK".to_string());
}
