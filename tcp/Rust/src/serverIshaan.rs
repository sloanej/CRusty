use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;
use std::io::{self, Read, Write, Error};

// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!("Message from client: {}", str::from_utf8(&buf).expect("Could not write buffer as string"));
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read from stdin");
        stream.write(input.as_bytes())?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
