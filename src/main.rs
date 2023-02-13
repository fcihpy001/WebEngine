use std::{fs, thread};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;
use crate::httprequest::{HttpRequest, Method, Resource, Version};
use crate::threadpool::ThreadPool;

pub mod data;
pub mod client;
pub mod server;
pub mod router;
pub mod middleware;
pub mod responder;
pub mod threadpool;
pub mod httprequest;
pub mod httpresponse;
pub mod handler;


type Result<T> = std::result::Result<T,dyn std::error::Error>;

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
        8081
    );
    // 服务端启动监听服务，绑定本机的ip址和端口号
    let listener = TcpListener::bind(&addr).unwrap();
    println!("服务启动成功: ip:{},port:{}",addr.ip(), addr.port());

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        // 使用线程池执行任务
        // pool.execute(|| {
        //     handle_stream(stream.unwrap());
        // });
        thread::spawn(move || {
            handle_stream(stream.unwrap());
        });
    }
}

// 处理访问请求
fn handle_stream(mut stream: TcpStream) {
    let mut route = HashMap::new();
    route.insert("/api", "handle_api");
    route.insert("/sleep", "handle_user");
    route.insert("/404", "handle_404");

    // println!("当前线程:{:#?}",thread::current());
    // println!("receive a new connection:: {}",stream.peer_addr().unwrap());

    let mut buffer = [0;1024];
    // 读取网络数据到缓冲区
    let length = stream.read(&mut buffer).unwrap();
    // 将缓冲区中的内容，转成字符串
    let request_str = String::from_utf8(Vec::from(&buffer[..length]));

    let request = HttpRequest::from(request_str.unwrap());
    println!("请求数据 {:#?}",request.resource);

    // 区分消息头和消息体的内容
    let response = if request.resource == Resource::Path("/api".to_string()) {
        println!("11");
        handle_api()
    } else if  request.resource == Resource::Path("/sleep".to_string())  {
        println!("22");

        handle_user()
    } else {
        println!("33");
       handle_404()
    };

    // 将数据返回到通道
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_api() -> String {
    let (status_line,filename) =  ("HTTP/1.1 200 OK\r\n\r\n", "index.html");
    // 组装响应内容
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status_line,content);
    response
}

fn handle_user() -> String {
    let (status_line,filename) =  ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html");
    // 组装响应内容
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status_line,content);
    response
}

fn handle_404() -> String {
    let (status_line,filename) =  ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html");

    // 组装响应内容
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status_line,content);
    response
}




