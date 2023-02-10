use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::threadpool::ThreadPool;

pub mod data;
pub mod client;
pub mod server;
pub mod router;
pub mod middleware;
pub mod responder;
pub mod threadpool;


fn main() {
    println!("Hello, world!");
    /*
    1.启动端口监听服务
    2.接收网络请求
    3.分析参数，get/post，消息头包含字段
    4.返回json信息
    5.多线程处理
     */

    // 要监听的ip地址和端口号
    let addr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(127,0,0,1)
        ),
        8088
    );
    // 服务端启动监听服务，绑定本机的ip址和端口号
    let listener = TcpListener::bind(&addr).unwrap();
    println!("服务启动成功: ip:{},port:{}",addr.ip(), addr.port());

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        // handle_stream(stream.unwrap());
        pool.execute(|| {
            handle_stream(stream.unwrap());
        })
    }
}

// 处理访问请求
fn handle_stream(mut stream: TcpStream) {
    println!("receive a new connection:: ");
    let mut buffer = [0;1024];
    // 读取网络数据到缓冲区
    let length = stream.read(&mut buffer).unwrap();
    // 将缓冲区中的内容，转成字符串
    let content = String::from_utf8_lossy(&buffer[..length]);
    println!("data:\n {content:?}");

    // 区分消息头和消息体的内容
    let get = b"GET / HTTP/1.1 \r\n";
    let sleep = b"GET /sleep HTTP/1.1 \r\n";

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status_line,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


