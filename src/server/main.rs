use std::io::{Bytes, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::Shutdown;
use std::thread;
use std::net::{TcpListener, TcpStream};

// const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 116);
const PORT: u16 = 8000;

fn main() {
    println!("Hello from Server!");
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();

    println!("{:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(err) => println!("Connection failed due to {:?}", err)
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8;60]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}