use std::{
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 512];
    connection.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer);

    println!("Got a message, which says {message}");

    let resp = b" Hello there, salutations from RUST";

    connection.write(resp).unwrap();
}

fn main() {
    let ip_port = "127.0.0.1:7878";
    let tcp_connection_listener = TcpListener::bind(ip_port).unwrap();

    println!("Rust server is running at {ip_port}");

    for connection in tcp_connection_listener.incoming() {
        match connection {
            Ok(connection) => {
                println!("New connection Received!");
                handle_connection(connection);
            }
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
    }

    println!("Hello, world!");
}
