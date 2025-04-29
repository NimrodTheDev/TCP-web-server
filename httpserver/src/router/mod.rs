use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::httprequest::Method;
use http::httprequest::Resource;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use std::net::TcpStream;
pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut TcpStream) -> () {
        match req.method {
            http::httprequest::Method::Get => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
