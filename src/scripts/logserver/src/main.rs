use std::net::TcpListener;

fn main() {

    ctrlc::set_handler(|| {

    });

    let mut srv = TcpListener::bind("localhost:8080").unwrap();


}
