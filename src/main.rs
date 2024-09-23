use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";
    let get_about = b"GET /about HTTP/1.1\r\n";

    // Vérifier la requête et répondre avec la bonne page
    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(get_about) {
        ("HTTP/1.1 200 OK", "public/about.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection établie!");

        // Créer un nouveau thread pour chaque connexion
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
