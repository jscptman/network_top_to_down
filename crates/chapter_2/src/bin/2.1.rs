use http_body_util::Empty;
use hyper::Request;
use hyper::client::conn::http1::handshake;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("example.com:80").await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, _conn) = handshake(io).await.unwrap();

    // 使用 sender 发送请求
    let req = Request::builder()
        .uri("/")
        .body(Empty::<&[u8]>::new())
        .unwrap();
    let response = sender.send_request(req).await.unwrap();

    // 处理响应
    println!("Response status: {}", response.status());
}
