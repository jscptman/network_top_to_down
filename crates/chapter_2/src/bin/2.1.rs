use std::io::{Read, Result as IoResult};
use std::net::{TcpListener, TcpStream};

enum HttpProtocol {
    HTTP11,
    HTTP2,
}
struct HttpMessage {
    protocol: HttpProtocol,
}
enum HttpParseError {}
impl HttpMessage {
    fn new<T>(message: T) -> Self
    where
        T: AsRef<str>,
    {
        Self::parse_message(message.as_ref())
            .unwrap_or_else(|e| panic!("http parse occurs an error"))
    }

    fn parse_message(message: &str) -> Result<Self, HttpParseError> {
        Ok(Self {
            protocol: HttpProtocol::HTTP11,
        })
    }
}

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
