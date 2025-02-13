use std::io::{Read, Result as IoResult};
use std::net::{TcpListener, TcpStream};

fn main() {
    start().unwrap_or_else(|e| {
        eprintln!("{}", e);
    })
}

fn start() -> IoResult<()> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    while let Ok((stream, addr)) = listener.accept() {
        println!("got a connection from: {}", addr);
        handle_request(stream)?
    }
    Ok(())
}

fn handle_request(mut req: TcpStream) -> IoResult<()> {
    let mut buf = String::new();
    let read_num = req.read_to_string(&mut buf)?;
    println!("request:\n{}", &buf);
    Ok(())
}
