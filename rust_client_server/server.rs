//https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    const BUFFER_SIZE: usize = 100000;

    let mut buffer: [u8 ; BUFFER_SIZE] = [0 ; BUFFER_SIZE];
    loop{
        match stream.read_exact(&mut buffer){
            Ok(()) => {
                //if size == 0 {return}
                //println!("{:?}", &buffer);
                /*for (index, num) in buffer.iter().enumerate() {
                    if *num != 4 as u8{
                        println!("bad{:?}", index);
                        break;
                    }
                }*/
                
            },
            Err(e) => {
                println!("{:?}",e);
                return Ok(());
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    // accept connections and process them serially
    Ok(for stream in listener.incoming() {
        match handle_client(stream?){
            Ok(()) => {return Ok(());},
            Err(e) => {return Err(e);}
        }
    })
}