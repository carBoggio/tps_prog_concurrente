mod calcular_pi_leibniz;
mod handle_connection;

use handle_connection::handle_connection;
use std::net::TcpListener;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(|| {
            handle_connection(stream); // Aquí calculás Pi y respondés [cite: 20]
        });
    }
}




