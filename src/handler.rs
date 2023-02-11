// use std::io::BufRead;
// use crate::httprequest::HttpRequest;
// use crate::httpresponse::HttpResponse;
//
// pub struct Handler {
//
// }
// impl Handler {
//    pub fn handle(req: &HttpRequest) -> HttpResponse {
//         let path = &req.resource;
//         let route = path.split("/").collect();
//         match route[1] {
//             "" => HttpResponse::new("200", None,None),
//             _ => {
//                 HttpResponse::new("404", None,None),
//             }
//         }
//     }
// }