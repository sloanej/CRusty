//https://doc.rust-lang.org/std/net/struct.TcpStream.html#impl-Read
use std::net::TcpStream;
use std::io::{Read, Write};
use std::process::exit;
use ctrlc;

fn main() -> std::io::Result<()> {
    ctrlc::set_handler(move || {
        println!("\nInsert metrics here :)");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");


    const BUFFER_SIZE: usize = 100000;

    let mut stream = TcpStream::connect("127.0.0.1:9000")?;

    let mut buffer: [u8 ; BUFFER_SIZE] = [0 ; BUFFER_SIZE];

    loop{
        stream.read_exact(&mut buffer)?;
        stream.write(&mut buffer)?;
    }

} // the stream is closed here