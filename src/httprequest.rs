// use std::collections::HashMap;
// use serde_json::Value::String;
//
// #[derive(Debug, PartialEq)]
// pub enum  Method {
//     Get,
//     Post,
//     Put,
//     Update,
//     Delete
// }
//
// impl From<&str> for Method {
//     fn from(value: &str) -> Self {
//         match value {
//             "GET" => Method::Get,
//             "POST" => Method::Post,
//             "PUT" => Method::Put,
//             "UPDATE" => Method::Update,
//             "DELETE" => Method::Delete,
//             _ => Method::Get
//         }
//     }
// }
//
// #[derive(Debug, PartialEq)]
// pub enum Version {
//     V11,
//     V20,
//     V30
// }
//
// impl From<&str> for Version {
//     fn from(value: &str) -> Self {
//         match value {
//             "HTTP/1.1" => Version::V11,
//             "HTTP/2.0" => Version::V20,
//             "HTTP/3.0" => Version::V30,
//             _ => Version::V11,
//         }
//     }
// }
//
// #[derive(Debug, PartialEq)]
// pub enum Resource {
//     Path(String)
// }
//
// #[derive(Debug)]
// pub struct HttpRequest {
//     pub method: Method,
//     pub version: Version,
//     pub resource: Resource,
//     pub headers: HashMap<String, String>,
//     pub msg_body: String
// }
//
// // 请求数据解析
// ///        "GET /index HTTP/1.1\r\n\
// //         Host: localhost\r\n\
// //         User-Agent: Curl/7.64.1\r\n\
// //         Accept: */*\r\n\r\n"
// //         name:"fcihpy"
// //         age: 10
// impl From<String> for HttpRequest {
//     fn from(value: String) -> Self {
//         let mut method = Method::Get;
//         let mut version = Version::V11;
//         let mut resource = Resource::Path("".to_string());
//         let mut headers = HashMap::new();
//         let mut msg_body = "";
//         for line in value.lines() {
//             if line.contains("HTTP") {
//                 // 请求头前三个字段，
//                 let (
//                     method,
//                     resource,
//                     version
//                 ) = process_req_line(line);
//             } else if line.contains(":") {
//                 // 请求头其它字段
//                 let (key, value) = process_header_line(line);
//                 headers.insert(key,value);
//             } else if line.len() == 0 {
//                 //没有请求体
//             } else {
//                 // 消息体
//                 msg_body = line
//             }
//         }
//         HttpRequest {
//             method,
//             resource,
//             version,
//             headers,
//             msg_body: msg_body.to_string()
//         }
//     }
// }
//
// // 处理请求体数据,格式如:  GET /index HTTP/1.1
// fn process_req_line(s: &str) -> (Method, Resource, Version) {
//     let mut words = s.split_whitespace();
//     let method = words.next().unwrap();
//     let resource = words.next().unwrap();
//     let version = words.next().unwrap();
//     (
//         method.into(),
//         Resource::Path(resource.to_string()),
//         version.into()
//     )
// }
//
// // 处理消息头数据
// fn process_header_line(s: &str) -> (String, String) {
//     let mut header_items = s.split(":");
//     let mut key = String::from("");
//     let mut value = String::from("");
//     if let Some(k) = header_items.next() {
//         key = k.to_string();
//     }
//     if let Some(v) = header_items.next() {
//         value = v.to_string();
//     }
//     (key, value)
// }
//
//
// #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn test_method_into() {
// //         let m: Method = "GET".into();
// //         assert_eq!(m, Method::Get);
// //     }
// //
// //     #[test]
// //     fn test_version_into() {
// //         let v: Version = "HTTP/1.1".into();
// //         assert_eq!(v, Version::V11);
// //     }
// //
// //     #[test]
// //     fn test_read_http() {
// //         let s: String = String::from("GET /index HTTP/1.1\r\n\
// //         Host: localhost\r\n\
// //         User-Agent: Curl/7.64.1\r\n\
// //         Accept: */*\r\n\r\n");
// //         let mut headers_expected = HashMap::new();
// //         headers_expected.insert("Host".into(), " localhost".into());
// //         headers_expected.insert("User-Agent".into(), " Curl/7.64.1".into());
// //         headers_expected.insert("Accept".into(), " */*".into());
// //         let req: HttpRequest = s.into();
// //
// //         assert_eq!(Method::Get, req.method);
// //         assert_eq!(Resource::Path("/index".to_string()), req.resource);
// //         assert_eq!(Version::V11, req.version);
// //         assert_eq!(headers_expected, req.headers);
// //     }
// // }
//
