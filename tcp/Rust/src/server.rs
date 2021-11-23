//https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::exit;
use ctrlc;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    const BUFFER_SIZE: usize = 100000;
    let mut buffer: [u8 ; BUFFER_SIZE] = [4 ; BUFFER_SIZE];
    
    loop{
        stream.write(&mut buffer)?;
        stream.read_exact(&mut buffer)?;
    }
}

fn main() -> std::io::Result<()> {
    ctrlc::set_handler(move || {
        println!("\nInsert metrics here :)");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let listener = TcpListener::bind("127.0.0.1:9000")?;

    // accept connections and process them serially
    Ok(for stream in listener.incoming() {
        match handle_client(stream?){
            Ok(()) => {return Ok(());},
            Err(e) => {return Err(e);}
        }
    })
}