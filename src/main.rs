use bson_module::{
    read_collection_deserialized, search_in_vector_document,
    store_document, string_to_document, serialize_collection_to_string,
};
use command_handler::Command;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
mod bson_module;
mod command_handler;

#[tokio::main]
async fn main() {
    //Creamos un TCPListener en el puerto 6379
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        //Utilizamos spawn para procesarlas "concurrently" (no me se la traduccion ahora xd)
        //Nota 1: mas o menos seria procesarlas "a la vez" xd
        //Basicamente no bloquear el uso del servidor TCP, que pueda servir a varios usuarios a la vez
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];
    println!("New connection");
    //Ponemos un bucle para leer de el socket y devolver la informacion
    loop {
        //Leemos el contenido del socket en buf y guardamos el length en n
        let n = socket
            .read(&mut buf)
            .await
            .expect("Failed to read data from socket");

        //si n esta vacia (no hay mensaje) hacemos return
        if n == 0 {
            return;
        }
        //Printeamos el buffer recibido, si hacemos unwrap cogemos solo el valor
        //Si hacemos sin eso nos devuelve Ok(mensaje) o Err(error)
        let socket_message = String::from_utf8(buf[0..n].to_vec()).unwrap();
        println!("Received: {:?}", socket_message);

        //Separamos el mensaje de socket por espacios
        let message_array: Vec<&str> = socket_message.split_whitespace().collect();

        if message_array.len() == 0 {
            return handle_error(&mut socket, "No command sent".to_string()).await;
        }
        //data contains the rest of the socket_message
        let data = message_array[1..message_array.len()].join("");
        //Cogemos el comando que se ha enviado
        //Y en caso de que no lo encontremos devolvemos que no se ha encontrado el comando
        //Intentabamos utilizar socket_message cuando lo habiamos pasado antes (en get_command)
        //Tenemos que pasarlo como borrow en los dos lados para que ninguna de las funciones coja el socket_message
        //como su propiedad!!!
        match command_handler::process_command(message_array[0]) {
            Ok(command) => handle_response(&mut socket, command, data).await,
            Err(e) => {
                //le pasamos &mut socket porque es como lo necesita (xd)
                handle_error(&mut socket, e).await;
            }
        }
    }
}

#[allow(dead_code)]
//Para poder concatenar dos String tenemos que hacer esta cosa rara xd
fn concat(s1: String, s2: String) -> String {
    s1 + &s2
}

async fn handle_error(socket: &mut TcpStream, error: String) {
    //& es para dejar utilizar la variable (borrow)
    println!("Error: {:?}", error.to_string());
    let buf = String::from(error.to_string()).into_bytes();
    socket
        .write_all(&buf[0..buf.len()])
        .await
        .expect("Failed to write error to socket");
}

async fn handle_response(socket: &mut TcpStream, command: Command, data: String) {
    println!("Received command: {:?}", command.to_string());
    //Si la variable solo se le asignara el valor una vez, no tiene porque ser mutable y no hace falta definirla
    //Si cambiara tiene que ser mut
    //Y si se usa antes de inicializarla, tiene que tener valor inicializado
    let response: String;
    //Para poder hacer el tema de Ok y Err, tenemos que llamar la funcion con match
    match command {
        Command::INSERT => match handle_insert(data).await {
            Ok(res) => response = res,
            Err(e) => response = e,
        },
        Command::FIND => match handle_find(data).await {
            Ok(res) => response = res,
            Err(e) => response = e,
        },
        Command::UPDATE => match handle_update(data).await {
            Ok(res) => response = res,
            Err(e) => response = e,
        },
    }
    println!("Sending: {:?}", response);
    let buf = response.into_bytes();
    socket
        .write_all(&buf[0..buf.len()])
        .await
        .expect("Failed to write response to socket");
}

async fn handle_insert(data: String) -> Result<String, String> {
    let document;

    match bson_module::string_to_document(data) {
        Ok(res) => document = res,
        Err(e) => return Err(e),
    }

    match bson_module::read_collection_deserialized().await {
        Ok(vec) => match store_document(document, vec).await {
            Ok(res) => return Ok(res),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

async fn handle_find(data: String) -> Result<String, String> {
    match string_to_document(data) {
        Ok(doc) => match read_collection_deserialized().await {
            Ok(vec) => {
                let found = search_in_vector_document(vec, doc);
                match serialize_collection_to_string(found) {
                    Ok(vec) => return Ok(vec),
                    Err(_e) => return Err("Failed to stringify BSON documents to JSON.".to_string()),
                }
            }
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    };
}

async fn handle_update(_data: String) -> Result<String, String> {
    return Err("Unimplemented".to_string());
}
