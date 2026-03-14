use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let request_line = {
        let buf_reader = BufReader::new(&mut stream);
        buf_reader.lines().next().unwrap().unwrap()
    };

    let parts: Vec<&str> = request_line.split(' ').collect();
    let method = parts.get(0).unwrap_or(&"");
    let path = parts.get(1).unwrap_or(&"/");

    let (status_line, contents) = if *method == "GET" && path.starts_with("/pi/") {
        // GET /pi/:i  -> extraer i del path
        let path_parts: Vec<&str> = path.split('/').collect();
        if path_parts.len() >= 3 {
            match path_parts[2].parse::<u64>() {
                Ok(i) => {
                    let pi = crate::calcular_pi_leibniz::calcular_pi_leibniz(i);
                    ("HTTP/1.1 200 OK", format!("{{\"pi\": {},\"iteraciones\": {}}}", pi, i))
                }
                Err(_) => ("HTTP/1.1 400 Bad Request", "{\"error\": \"i debe ser un número\"}".to_string()),
            }
        } else {
            ("HTTP/1.1 400 Bad Request", "{\"error\": \"Uso: GET /pi/<número>\"}".to_string())
        }
    } else {
        ("HTTP/1.1 404 Not Found", "{\"error\": \"Ruta no encontrada\"}".to_string())
    };

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
