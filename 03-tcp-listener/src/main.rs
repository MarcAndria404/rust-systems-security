use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let adress = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(&adress).unwrap();
    let mut buffer = [0u8; 1024];
    let shared: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream_result in listener.incoming() {
        let shared = Arc::clone(&shared);
        match stream_result {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("New connexion");
                    let Ok(stream_clone) = stream.try_clone() else {
                        println!("Error");
                        return;
                    };
                    shared.lock().unwrap().push(stream_clone);
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            println!("{}", String::from_utf8_lossy(&buffer[..n]));
                            let mut guard = shared.lock().unwrap();
                            for stream_in_list in guard.iter_mut() {
                                if let Err(e) = stream_in_list.write_all(&buffer[..n]) {
                                    println!("Error {}", e);
                                };
                            }
                            
                        }
                        Err(e) => println!("Error {}", e),
                    }
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
