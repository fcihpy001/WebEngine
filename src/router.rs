// use std::io::Write;
// use tokio::io::AsyncBufReadExt;
// use crate::handler::Handler;
// use crate::httprequest::{HttpRequest, Method, Resource};
// use crate::httpresponse::HttpResponse;
//
// pub struct Router;
//
// impl Router {
//     pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
//         match req.method {
//             Method::Get => match &req.resource {
//                 Resource::Path(s) => {
//                     let route = s.split("/").collect();
//                     match route[1] {
//                         "api" => {
//                             let resp = Handler::handle(&req);
//                             let _ = resp.send_response(stream);
//
//                         },
//                         _ => {
//                             let resp = Handler::handle(&req);
//                             let _ = resp.send_response(stream);
//
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 let resp = Handler::handle(&req);
//                 let _ = resp.send_response(stream);
//             }
//         }
//     }
// }