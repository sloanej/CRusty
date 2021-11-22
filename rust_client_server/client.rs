//https://doc.rust-lang.org/std/net/struct.TcpStream.html#impl-Read
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};



fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;


    let mut buffer: [u8;10] = [4;10];
    stream.write(&mut buffer)?;
    Ok(())
} // the stream is closed here