#![deny(warnings)]

use http_body_util::{BodyExt, Full, combinators::BoxBody};
/// 作业1: Web服务器
/// 要求：
/// 1）当客户端联系时创建一个连接套接字；
/// 2）从这个连接套接字接收HTTP请求；
/// 3）解释该请求以确定所请求的特定文件；
/// 4）从服务器的文件系统获得所请求的文件；
/// 5）创建一个由请求的文件组成的HTTP响应报文，报文前面有首部行；
/// 6）经TCP连接向请求的浏览器发送响应。如果浏览器请求一个该服务器中不存在的文件，服务器应返回一个“404 Not Found”差错报文。
use hyper::{
    Method, Request, Response, Result as HyperResult, StatusCode,
    body::{self as hyper_body, Bytes},
    server::conn::http2,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::{net::TcpListener, task};

static BIND_ADDR: &str = "127.0.0.1:8080";
static NOT_FOUND: &str = "file not found";

#[derive(Clone)]
struct Http2Executor;
impl<F> hyper::rt::Executor<F> for Http2Executor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        task::spawn(fut);
    }
}

type ResponseType = Response<BoxBody<Bytes, Infallible>>;
async fn router(req: Request<hyper_body::Incoming>) -> HyperResult<ResponseType> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index") => Ok(handle_404()),
        _ => todo!(),
    }
}

fn handle_404() -> ResponseType {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::<Bytes>::from(NOT_FOUND).boxed())
        .unwrap()
}

fn _send_file(_filename: &str) -> ResponseType {
    // 根据文件名入参，传输文件字节流，设置对应的响应头实现下载操作
    todo!();
}
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(BIND_ADDR)
        .await
        .unwrap_or_else(|e| panic!("listen to {BIND_ADDR} occurs an error: {e}"));
    loop {
        let (stream, address) = listener
            .accept()
            .await
            .unwrap_or_else(|e| panic!("accept a request occurs an error: {e}"));
        println!("connected with {address}");
        let io = TokioIo::new(stream);
        task::spawn(async move {
            if let Err(e) = http2::Builder::new(Http2Executor)
                .serve_connection(io, service_fn(router))
                .await
            {
                panic!("serve_connection occurs an error: {e}")
            }
        });
    }
}
