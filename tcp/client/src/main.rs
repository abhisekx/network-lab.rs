use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

fn handle_server(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        println!("Enter message:");
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            return Ok(());
        }
        stream.write_all(input.as_bytes())?;

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Server disconnected.");
            return Ok(());
        };
        println!("Server: {}", String::from_utf8_lossy(&buffer[..n]).trim());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8888")?;
    println!("Connected to server at {}", stream.peer_addr()?);

    if let Err(e) = handle_server(stream) {
        println!("Connection error: {e}");
    }

    Ok(())
}
