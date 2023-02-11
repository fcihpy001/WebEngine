// use std::collections::HashMap;
// use std::io::Write;
// use reqwest::Request;
// use serde_json::Value::String;
//
// // 响应结体结构
// pub struct JsonResponse<T> {
//     code: u16,
//     msg: Option<String()>,
//     data: Option<T>
// }
//
// impl JsonResponse<T> {
//     pub fn success(data: Option<T>) -> Self {
//         Self {
//             code: 0,
//             msg: Some("success".to_string()),
//             data
//         }
//     }
//     pub fn fail(code: u16, msg: String) -> Self {
//         Self {
//             code,
//             msg,
//             data: None
//         }
//     }
// }
//
// #[derive(Debug,PartialEq,Clone)]
// pub struct HttpResponse<'a> {
//     version: &'a str,
//     status_code: &'a str,
//     status_text: &'a str,
//     headers: Option<HashMap<&'a str, &'a str>>,
//     body: Option<String>
// }
//
// impl<'a> Default for HttpResponse<'a> {
//     fn default() -> Self {
//         Self {
//             version: "HTTP/1.1".into(),
//             status_code: "200".into(),
//             status_text: "OK".into(),
//             headers: None,
//             body: None
//         }
//     }
// }
//
// impl<'a> From<HttpResponse<'a>> for String {
//     fn from(value: HttpResponse<'a>) -> Self {
//         let res = value.clone();
//         format!(
//             "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
//             &res.version(),
//             &res.status_code(),
//             &res.status_text(),
//             &res.headers(),
//             &res.body.unwrap().len(),
//             &res.body()
//         )
//     }
// }
//
// impl<'a> HttpResponse<'a> {
//     pub fn new(
//         status_code: &'a str,
//         headers: Option<HashMap<&'a str, &'a str>>,
//         body: Option<String>
//     ) -> Self<'a> {
//         let mut response = HttpResponse::default();
//         if status_code != "200" {
//             response.status_code = status_code.into();
//         }
//         response.headers = match &headers {
//             Some(_h) => headers,
//             None => {
//                 let mut h = HashMap::new();
//                 h.insert("Content-Type", "text/html");
//                 Some(h)
//             }
//         };
//         response.status_text = match response.status_code {
//             "200" => "OK".into(),
//             "400" => "Bad Request".into(),
//             "404" => "Not Found".into(),
//             "500" => "Internal Server Error".into(),
//             _ => "Not Found".into(),
//         };
//         response.body = body;
//         response
//     }
//
//     pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
//         let res = self.clone();
//         let response_string = String::from(res);
//         let _ = write!(write_stream, "{}", response_string);
//         Ok(())
//     }
//
//     fn version(&self) -> &str {
//         self.version
//     }
//
//     fn status_code(&self) -> &str {
//         self.status_code
//     }
//
//     fn status_text(&self) -> &str {
//         self.status_text
//     }
//
//     fn headers(&self) -> String {
//         let map = self.headers.clone().unwrap();
//         let mut header_string: String = "".into();
//         for (k,v) in map.iter() {
//             header_string = format!("{}{}:{}\r\n", header_string,k,v);
//         }
//         header_string
//     }
//
//     pub fn body(&self) -> &str {
//         match &self.body {
//             Some(b) => b.as_str(),
//             None => ""
//         }
//     }
// }
//
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn test_response_struct_creation_200() {
// //         let response_actual = HttpResponse::new(
// //             "200",
// //             None,
// //             Some("nothing for now".into()),
// //         );
// //         let response_expected = HttpResponse {
// //             version: "HTTP/1.1",
// //             status_code: "200",
// //             status_text: "OK",
// //             headers: {
// //                 let mut h = HashMap::new();
// //                 h.insert("Content-Type", "text/html");
// //                 Some(h)
// //             },
// //             body: Some("nothing for now".into()),
// //         };
// //
// //         assert_eq!(response_actual, response_expected);
// //     }
// //
// //     #[test]
// //     fn test_response_struct_creation_404() {
// //         let response_actual = HttpResponse::new(
// //             "404",
// //             None,
// //             Some("nothing for now".into()),
// //         );
// //         let response_expected = HttpResponse {
// //             version: "HTTP/1.1",
// //             status_code: "404",
// //             status_text: "Not Found",
// //             headers: {
// //                 let mut h = HashMap::new();
// //                 h.insert("Content-Type", "text/html");
// //                 Some(h)
// //             },
// //             body: Some("nothing for now".into()),
// //         };
// //
// //         assert_eq!(response_actual, response_expected);
// //     }
// //
// //     #[test]
// //     fn test_http_response_creation() {
// //         let response_expected = HttpResponse {
// //             version: "HTTP/1.1",
// //             status_code: "404",
// //             status_text: "Not Found",
// //             headers: {
// //                 let mut h = HashMap::new();
// //                 h.insert("Content-Type", "text/html");
// //                 Some(h)
// //             },
// //             body: Some("nothing for now".into()),
// //         };
// //         let http_string: String = response_expected.into();
// //         let actual_string: String =
// //             "HTTP/1.1 404 Not Found\r\n\
// //             Content-Type:text/html\r\n\
// //             Content-Length: 15\r\n\r\n\
// //             nothing for now".into(); // 此处注意Content-Length值
// //
// //         assert_eq!(http_string, actual_string);
// //     }
// // }
