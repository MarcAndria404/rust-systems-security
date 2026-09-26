use std::env;
use std::io::Read;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn port_scanner(ip_addr: String, min_port: u16, max_port: u16) {
    let Ok(ip) = ip_addr.parse::<IpAddr>() else {
        println!("invalid address");
        return;
    };

    let duration = Duration::new(1, 0);
    let mut handles = Vec::new();

    for port in min_port..=max_port {
        let socket_addr = SocketAddr::new(ip, port);
        handles.push(thread::spawn(move || {
            let result_connexion = TcpStream::connect_timeout(&socket_addr, duration);
            match result_connexion {
                Ok(mut stream) => {
                    let mut buffer = [0u8; 1024];
                    let Ok(()) = stream.set_read_timeout(Some(duration)) else {
                        println!("Error while reading buffer");
                        return;
                    };
                    match stream.read(&mut buffer) {
                        Ok(n) => println!(
                            "Port: {}, Banner: {}",
                            port,
                            String::from_utf8_lossy(&buffer[..n])
                        ),
                        Err(e) => println!("{}", e),
                    }
                }
                Err(_) => {}
            }
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let mut ip_addr = String::new();
    let mut min_port: u16 = 1;
    let mut max_port: u16 = 1024;

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 4 || args[1] == "--help" {
        println!("Help: `cargo run -- --help`");
        println!("Usage: `cargo run -- ip_addr min_port max_port`");
        return;
    } else {
        ip_addr = args[1].clone();
        min_port = args[2].parse::<u16>().expect("Failed");
        max_port = args[3].parse::<u16>().expect("Failed");
    }

    let start = Instant::now();
    port_scanner(ip_addr, min_port, max_port);
    let elapsed = start.elapsed();
    println!("{} ports scanned in {:?}", max_port, elapsed);
}
