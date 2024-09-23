use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("public/index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = "404 - Page not found";
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
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
