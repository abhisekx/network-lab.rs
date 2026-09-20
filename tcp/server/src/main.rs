use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Client connected from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Client disconnected (EOF).");
            return Ok(());
        }
        println!("Client: {}", String::from_utf8_lossy(&buffer[..n]).trim());

        stream.write_all(&buffer[..n])?;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8888")?;
    println!("Server listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let stream = stream?;
        if let Err(e) = handle_client(stream) {
            println!("Connection error: {e}");
        }
    }

    Ok(())
}
