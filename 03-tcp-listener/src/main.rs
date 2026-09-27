use std::net::TcpListener;
use std::io::prelude::*;
use std::thread;



fn main() {
    let adress = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(&adress).unwrap();
    let mut buffer = [0u8; 1024];
    let data = b"hello world";


    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("New connexion");
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            println!("{}", String::from_utf8_lossy(&buffer[..n]));
                            let bytes_written = stream.write_all(data);
                        },
                        Err(e) => println!("Error {}", e)
                    }
                     
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}