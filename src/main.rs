use orchestrator::{load_orchestrator, Orchestrator, User};

use std::{ops::Add, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use handlers::{
    command_handler, handle_create, handle_find, handle_insert, handle_peek, handle_update, Command,
};

use crate::handlers::{handle_delete, handle_register};

mod bson_module;
mod handlers;
mod orchestrator;

#[tokio::main]
async fn main() {
    //Creamos un TCPListener en el puerto 6379
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await.unwrap();
    let orchestrator;

    match load_orchestrator() {
        Ok(data) => {
            println!("Orch: {:?}", data);
            orchestrator = Arc::new(data);
        }
        Err(_e) => {
            println!("ERROR: There's been an error reading the Orchestrator");
            return;
        }
    }

    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let orchestrator = (*orchestrator).clone();
        //Utilizamos spawn para procesarlas "concurrently" (no me se la traduccion ahora xd)
        //Nota 1: mas o menos seria procesarlas "a la vez" xd
        //Basicamente no bloquear el uso del servidor TCP, que pueda servir a varios usuarios a la vez
        tokio::spawn(async move {
            //Getting URI
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            let initial_data = String::from_utf8(buffer[0..n].to_vec()).unwrap();
            //Pasamos el username:password a variables
            let mut split = initial_data.splitn(2, ':');
            let username = split.next().unwrap_or("").trim().to_string();
            let password = split.next().unwrap_or("").trim().to_string();
            //autentificamos el usuario
            match orchestrator.authenticate_user(&username, &password) {
                Ok(user) => process(socket, orchestrator, user).await,
                Err(error) => {
                    handle_error(&mut socket, error).await;
                    //println!("Closing connection");
                    match socket.shutdown().await {
                        Ok(_ok) => {
                            //println!("Connection closed.");
                            return;
                        }
                        Err(e) => return println!("ERROR: {:?}", e),
                    }
                }
            }
        });
    }
}

async fn process(mut socket: TcpStream, mut orchestrator: Orchestrator, user: User) {
    let mut buf = vec![0; 1024];
    println!("New connection");
    send_response(&mut socket, "Connected to the Database".into()).await;

    //si fem return, es "tanca" la conexio pero el front segueix conectat, per això hem de fer continue!!
    //Ponemos un bucle para leer de el socket y devolver la informacion

    //Leemos el contenido del socket en buf y guardamos el length en n
    while let Ok(n) = socket.read(&mut buf).await {
        //si n esta vacia (no hay mensaje) hacemos return
        if n == 0 {
            continue;
        }
        //Printeamos el buffer recibido, si hacemos unwrap cogemos solo el valor
        //Si hacemos sin eso nos devuelve Ok(mensaje) o Err(error)
        let socket_message = String::from_utf8(buf[0..n].to_vec()).unwrap();
        println!("Received: {:?}", socket_message);

        //Separamos el mensaje de socket por espacios
        let message_array: Vec<&str> = socket_message.split_whitespace().collect();

        if message_array.len() == 0 {
            handle_error(&mut socket, "No command sent".to_string()).await;
            continue;
        }

        //data contains the rest of the socket_message
        let data = &message_array[1..message_array.len()];

        //Cogemos el comando que se ha enviado
        //Y en caso de que no lo encontremos devolvemos que no se ha encontrado el comando
        //Intentabamos utilizar socket_message cuando lo habiamos pasado antes (en get_command)
        //Tenemos que pasarlo como borrow en los dos lados para que ninguna de las funciones coja el socket_message
        //como su propiedad!!!
        match command_handler::process_command(message_array[0]) {
            Ok(command) => {
                handle_response(
                    &mut socket,
                    command,
                    data.to_owned(),
                    &mut orchestrator,
                    &user,
                )
                .await
            }
            Err(e) => {
                //le pasamos &mut socket porque es como lo necesita (xd)
                handle_error(&mut socket, e).await;
            }
        }
    }
    //Si se desconecta, que devuelva y ya
    return;
}

async fn handle_error(socket: &mut TcpStream, error: String) {
    //& es para dejar utilizar la variable (borrow)
    println!("Error: {:?}", error.to_string());
    let buf = String::from(error.to_string()).into_bytes();
    socket
        .write_all(&buf[0..buf.len()])
        .await
        .expect("Failed to write error to socket");
    return;
}

async fn handle_response(
    socket: &mut TcpStream,
    command: Command,
    data: Vec<&str>,
    orchestrator: &mut Orchestrator,
    user: &User,
) {
    println!("Received command: {:?}", command.to_string());
    //Si la variable solo se le asignara el valor una vez, no tiene porque ser mutable y no hace falta definirla
    //Si cambiara tiene que ser mut
    //Y si se usa antes de inicializarla, tiene que tener valor inicializado
    let response: String;
    //Para poder hacer el tema de Ok y Err, tenemos que llamar la funcion con match
    //let (database, collection, content) = get_data(data);

    match command {
        Command::INSERT => match handle_insert(&data, orchestrator, user).await {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::FIND => match handle_find(&data, orchestrator, user).await {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::UPDATE => match handle_update(&data, orchestrator, user).await {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::CREATE => match handle_create(&data, orchestrator, user) {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::DELETE => match handle_delete(&data, orchestrator, user).await {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::PEEK => match handle_peek(&data, orchestrator, user) {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
        Command::REGISTER => match handle_register(&data.join(""), orchestrator, user) {
            Ok(res) => response = res,
            Err(e) => response = "ERROR: ".to_owned().add(&e),
        },
    }
    return send_response(socket, response).await;
}

async fn send_response(socket: &mut TcpStream, response: String) {
    println!("Sending: {:?}", response);
    let buf = response.into_bytes();
    socket
        .write_all(&buf[0..buf.len()])
        .await
        .expect("Failed to write response to socket");
}
