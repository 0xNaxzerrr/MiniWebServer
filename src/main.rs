use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Requête reçue: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // On écoute sur le port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Boucle infinie pour accepter des connexions
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection établie!");

        // Appeler une fonction pour traiter chaque connexion
        handle_connection(stream);
    }
}
