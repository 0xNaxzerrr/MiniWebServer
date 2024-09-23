use env_logger;
use log::{info, warn};
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    info!("Requête reçue : {}", String::from_utf8_lossy(&buffer[..]));

    let get_root = b"GET / HTTP/1.1\r\n";
    let get_about = b"GET /about HTTP/1.1\r\n";

    // Vérifier la requête et répondre avec la bonne page
    let (status_line, filename) = if buffer.starts_with(get_root) {
        info!("Route / trouvée");
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(get_about) {
        info!("Route /about trouvée");
        ("HTTP/1.1 200 OK", "public/about.html")
    } else {
        warn!("Route non trouvée");
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
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Pool de 4 threads
    info!("Serveur en écoute sur 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("Nouvelle connexion établie!");

        // On passe chaque connexion au pool de threads
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
