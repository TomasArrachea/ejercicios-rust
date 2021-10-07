use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_cliente = thread::spawn(move || {
        let mut sender = TcpStream::connect("localhost:7878").unwrap();

        sender.write("Hola desde el cliente.".as_bytes()).unwrap();

        let mut buffer = [0; 1024];
        sender.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
    });

    let (mut stream, _addr) = listener.accept().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    stream.write("Hola desde el servidor.".as_bytes()).unwrap();

    thread_cliente.join().unwrap();
}
