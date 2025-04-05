use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, webServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::path(s) => {
                    let route = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = webServiceHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handler(&req);
                            let _ = resp.send_message(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handler(&req);
                let _ = res.send_message(stream);
            }
        }
    }
}
