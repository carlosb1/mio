use mio::net::{TcpListener, TcpStream};
use mio::Token;
use std::io::Result;

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

// Some data we'll send over the connection.
const DATA: &[u8] = b"Hello world!\n";

fn main() -> Result<()> {
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut listener = TcpListener::bind(addr)?;
    let mut stream = TcpStream::connect(listener.local_addr()?)?;
    let mut std_stream = stream.to_std();
    Ok(())
}
