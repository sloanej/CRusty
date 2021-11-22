//https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8;10] = [0;10];
    stream.read(&mut buffer);
    println!("{:?}", &buffer);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}