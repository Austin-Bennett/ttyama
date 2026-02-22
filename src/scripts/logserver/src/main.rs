use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    println!("Incoming connection from: {}", addr);

    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let timestamp = chrono::Local::now();
        println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
}


fn main() {

    let quit = Arc::new(AtomicBool::new(false));
    {
        let q = quit.clone();
        ctrlc::set_handler(move || {
            println!("Received SIGINT, shutting down server...");
            q.store(true, std::sync::atomic::Ordering::Release);
        }).unwrap();
    }


    let mut srv = TcpListener::bind("localhost:8080").unwrap();
    println!("Listening for connections on port 8080");
    srv.set_nonblocking(true).unwrap();

    loop {
        match srv.accept() {
            Ok((socket, addr)) => {
                thread::spawn(
                    move || {
                        handle_connection(socket, addr);
                    }
                );
            },
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if quit.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("accept error: {:?}", e);
            }
        }
    }
}
